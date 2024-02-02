use io::Write;
use std::{io, ptr::NonNull, str};

struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

trait Cluster: Clone {
    type V: Default + Copy + std::fmt::Debug;

    fn identity() -> Self;
    fn compress(left: Self, right: Self, a: Self::V, b: Self::V, c: Self::V) -> Self;
    fn rake(left: Self, right: Self, a: Self::V, b: Self::V, c: Self::V) -> Self;
    fn reverse(&mut self);
}

type Link<N> = Option<N>;

struct VertexRaw<T: Cluster> {
    val: T::V,
    handle: Option<CompNode<T>>,
}

impl<T: Cluster> VertexRaw<T> {
    fn new(val: T::V) -> Self {
        VertexRaw { val, handle: None }
    }

    fn dummy() -> Self {
        VertexRaw {
            val: T::V::default(),
            handle: None,
        }
    }

    fn handle(&self) -> Option<CompNode<T>> {
        self.handle
    }

    fn handle_mut(&mut self) -> &mut Option<CompNode<T>> {
        &mut self.handle
    }

    fn value(&self) -> T::V {
        self.val
    }

    fn set_value(&mut self, val: T::V) {
        self.val = val;
    }
}

struct Vertex<T: Cluster> {
    vertex: NonNull<VertexRaw<T>>,
}

impl<T: Cluster> Vertex<T> {
    fn dangling() -> Self {
        Vertex {
            vertex: NonNull::dangling(),
        }
    }

    fn new(val: T::V) -> Self {
        unsafe {
            let v = Vertex {
                vertex: NonNull::new_unchecked(Box::into_raw(Box::new(VertexRaw::new(val)))),
            };
            let dummy = Vertex {
                vertex: NonNull::new_unchecked(Box::into_raw(Box::new(VertexRaw::dummy()))),
            };

            link(v, dummy, T::identity());

            v
        }
    }

    fn handle(&self) -> Option<CompNode<T>> {
        unsafe { self.vertex.as_ref().handle() }
    }

    fn handle_mut(&mut self) -> &mut Option<CompNode<T>> {
        unsafe { self.vertex.as_mut().handle_mut() }
    }

    fn value(&self) -> T::V {
        unsafe { self.vertex.as_ref().value() }
    }

    fn set_value(&mut self, val: T::V) {
        unsafe {
            self.vertex.as_mut().set_value(val);
        }
    }
}

impl<T: Cluster> Clone for Vertex<T> {
    fn clone(&self) -> Self {
        Vertex {
            vertex: self.vertex.clone(),
        }
    }
}
impl<T: Cluster> Copy for Vertex<T> {}
impl<T: Cluster> PartialEq for Vertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.vertex == other.vertex
    }
}

enum CompNode<T: Cluster> {
    Node(NonNull<Compress<T>>),
    Leaf(NonNull<Edge<T>>),
}

enum RakeNode<T: Cluster> {
    Node(NonNull<Rake<T>>),
    Leaf(CompNode<T>),
}

enum ParentNode<T: Cluster> {
    Compress(NonNull<Compress<T>>),
    Rake(NonNull<Rake<T>>),
}

struct Edge<T: Cluster> {
    v: [Vertex<T>; 2],
    par: Link<ParentNode<T>>,
    me: NonNull<Edge<T>>,
    val: T,
}

struct Compress<T: Cluster> {
    ch: [CompNode<T>; 2],
    v: [Vertex<T>; 2],
    rake: Link<RakeNode<T>>,
    par: Link<ParentNode<T>>,
    me: NonNull<Compress<T>>,
    rev: bool,

    guard: bool,
    fold: T,
}

struct Rake<T: Cluster> {
    ch: [RakeNode<T>; 2],
    v: [Vertex<T>; 2],
    par: Link<ParentNode<T>>,

    fold: T,
}

impl<T: Cluster> Edge<T> {
    fn new(v: Vertex<T>, u: Vertex<T>, val: T) -> NonNull<Edge<T>> {
        unsafe {
            let mut e = NonNull::new_unchecked(Box::into_raw(Box::new(Edge {
                v: [v, u],
                par: None,
                val: val,
                me: NonNull::dangling(),
            })));

            e.as_mut().me = e;
            e.as_mut().fix();

            e
        }
    }
}

impl<T: Cluster> Compress<T> {
    fn new(left: CompNode<T>, right: CompNode<T>) -> NonNull<Compress<T>> {
        unsafe {
            let mut n = NonNull::new_unchecked(Box::into_raw(Box::new(Compress {
                ch: [left, right],
                v: [Vertex::dangling(), Vertex::dangling()],
                rake: None,
                par: None,
                rev: false,
                me: NonNull::dangling(),
                guard: false,
                fold: T::identity(),
            })));

            n.as_mut().me = n;
            n.as_mut().fix();

            n
        }
    }
}

impl<T: Cluster> Rake<T> {
    fn new(left: RakeNode<T>, right: RakeNode<T>) -> NonNull<Rake<T>> {
        unsafe {
            let mut r = NonNull::new_unchecked(Box::into_raw(Box::new(Rake {
                ch: [left, right],
                v: [Vertex::dangling(), Vertex::dangling()],
                par: None,
                fold: T::identity(),
            })));

            r.as_mut().fix();

            r
        }
    }
}

trait TVertex<T: Cluster> {
    fn fix(&mut self);
    fn push(&mut self);
    fn reverse(&mut self);
    fn parent(&self) -> Link<ParentNode<T>>;
    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>>;
}

trait Node<T: Cluster>: TVertex<T> {
    type Child: TVertex<T>;

    fn child(&self, dir: usize) -> Self::Child;
    fn child_mut(&mut self, dir: usize) -> &mut Self::Child;
}

impl<T: Cluster> TVertex<T> for Edge<T> {
    fn fix(&mut self) {
        match self.parent() {
            Some(ParentNode::Compress(_)) => {
                if parent_dir_comp(CompNode::Leaf(self.me)).is_none() {
                    *self.v[0].handle_mut() = Some(CompNode::Leaf(self.me));
                }
            }
            Some(ParentNode::Rake(_)) => {
                *self.v[0].handle_mut() = Some(CompNode::Leaf(self.me));
            }
            None => {
                *self.v[0].handle_mut() = Some(CompNode::Leaf(self.me));
                *self.v[1].handle_mut() = Some(CompNode::Leaf(self.me));
            }
        }
    }

    fn push(&mut self) {}

    fn reverse(&mut self) {
        self.v.swap(0, 1);
        self.val.reverse();
    }

    fn parent(&self) -> Link<ParentNode<T>> {
        self.par
    }

    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        &mut self.par
    }
}

impl<T: Cluster> Compress<T> {
    fn rake(&self) -> Link<RakeNode<T>> {
        self.rake
    }

    fn rake_mut(&mut self) -> &mut Link<RakeNode<T>> {
        &mut self.rake
    }
}

impl<T: Cluster> TVertex<T> for Compress<T> {
    fn fix(&mut self) {
        self.push();
        self.v[0] = self.ch[0].endpoints(0);
        self.v[1] = self.ch[1].endpoints(1);

        self.fold = T::compress(
            match self.rake {
                Some(r) => T::rake(
                    self.ch[0].fold(),
                    r.fold(),
                    self.ch[0].endpoints(0).value(),
                    r.endpoints(0).value(),
                    self.ch[0].endpoints(1).value(),
                ),
                None => self.ch[0].fold(),
            },
            self.ch[1].fold(),
            self.ch[0].endpoints(0).value(),
            self.ch[1].endpoints(1).value(),
            self.ch[0].endpoints(1).value(),
        );

        *self.ch[0].endpoints(1).handle_mut() = Some(CompNode::Node(self.me));

        match self.parent() {
            Some(ParentNode::Compress(_)) => {
                if parent_dir_comp(CompNode::Node(self.me)).is_none() {
                    *self.v[0].handle_mut() = Some(CompNode::Node(self.me));
                }
            }
            Some(ParentNode::Rake(_)) => {
                *self.v[0].handle_mut() = Some(CompNode::Node(self.me));
            }
            _ => {
                *self.v[0].handle_mut() = Some(CompNode::Node(self.me));
                *self.v[1].handle_mut() = Some(CompNode::Node(self.me));
            }
        }
    }

    fn push(&mut self) {
        if self.rev {
            self.ch.swap(0, 1);
            self.ch[0].reverse();
            self.ch[1].reverse();
            self.rev = false;
        }
    }

    fn reverse(&mut self) {
        self.v.swap(0, 1);
        self.fold.reverse();
        self.rev ^= true;
    }

    fn parent(&self) -> Link<ParentNode<T>> {
        self.par
    }

    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        &mut self.par
    }
}

impl<T: Cluster> Node<T> for Compress<T> {
    type Child = CompNode<T>;

    fn child(&self, dir: usize) -> Self::Child {
        self.ch[dir]
    }

    fn child_mut(&mut self, dir: usize) -> &mut Self::Child {
        &mut self.ch[dir]
    }
}

impl<T: Cluster> TVertex<T> for Rake<T> {
    fn fix(&mut self) {
        self.push();
        self.v = [self.ch[0].endpoints(0), self.ch[0].endpoints(1)];
        self.fold = T::rake(
            self.ch[0].fold(),
            self.ch[1].fold(),
            self.ch[0].endpoints(0).value(),
            self.ch[1].endpoints(0).value(),
            self.ch[0].endpoints(1).value(),
        );
    }

    fn push(&mut self) {}

    fn reverse(&mut self) {}

    fn parent(&self) -> Link<ParentNode<T>> {
        self.par
    }

    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        &mut self.par
    }
}

impl<T: Cluster> Node<T> for Rake<T> {
    type Child = RakeNode<T>;

    fn child(&self, dir: usize) -> Self::Child {
        self.ch[dir]
    }

    fn child_mut(&mut self, dir: usize) -> &mut Self::Child {
        &mut self.ch[dir]
    }
}

impl<T: Cluster> CompNode<T> {
    fn endpoints(&self, dir: usize) -> Vertex<T> {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().v[dir],
                CompNode::Leaf(leaf) => leaf.as_ref().v[dir],
            }
        }
    }

    fn fold(&self) -> T {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().fold.clone(),
                CompNode::Leaf(leaf) => leaf.as_ref().val.clone(),
            }
        }
    }
}

impl<T: Cluster> RakeNode<T> {
    fn endpoints(&self, dir: usize) -> Vertex<T> {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().v[dir],
                RakeNode::Leaf(leaf) => leaf.endpoints(dir),
            }
        }
    }

    fn fold(&self) -> T {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().fold.clone(),
                RakeNode::Leaf(leaf) => leaf.fold(),
            }
        }
    }
}

impl<T: Cluster> TVertex<T> for CompNode<T> {
    fn fix(&mut self) {
        unsafe {
            match *self {
                CompNode::Node(mut node) => node.as_mut().fix(),
                CompNode::Leaf(mut leaf) => leaf.as_mut().fix(),
            }
        }
    }

    fn push(&mut self) {
        unsafe {
            match *self {
                CompNode::Node(mut node) => node.as_mut().push(),
                CompNode::Leaf(mut leaf) => leaf.as_mut().push(),
            }
        }
    }

    fn reverse(&mut self) {
        unsafe {
            match *self {
                CompNode::Node(mut node) => node.as_mut().reverse(),
                CompNode::Leaf(mut leaf) => leaf.as_mut().reverse(),
            }
        }
    }

    fn parent(&self) -> Link<ParentNode<T>> {
        unsafe {
            match *self {
                CompNode::Node(node) => node.as_ref().parent(),
                CompNode::Leaf(leaf) => leaf.as_ref().parent(),
            }
        }
    }

    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        unsafe {
            match self {
                CompNode::Node(ref mut node) => node.as_mut().parent_mut(),
                CompNode::Leaf(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}

impl<T: Cluster> TVertex<T> for RakeNode<T> {
    fn fix(&mut self) {
        unsafe {
            match *self {
                RakeNode::Node(mut node) => node.as_mut().fix(),
                RakeNode::Leaf(mut leaf) => leaf.fix(),
            }
        }
    }

    fn push(&mut self) {
        unsafe {
            match *self {
                RakeNode::Node(mut node) => node.as_mut().push(),
                RakeNode::Leaf(mut leaf) => leaf.push(),
            }
        }
    }

    fn reverse(&mut self) {
        unsafe {
            match *self {
                RakeNode::Node(mut node) => node.as_mut().reverse(),
                RakeNode::Leaf(mut leaf) => leaf.reverse(),
            }
        }
    }

    fn parent(&self) -> Link<ParentNode<T>> {
        unsafe {
            match *self {
                RakeNode::Node(node) => node.as_ref().parent(),
                RakeNode::Leaf(leaf) => leaf.parent(),
            }
        }
    }

    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        unsafe {
            match self {
                RakeNode::Node(ref mut node) => node.as_mut().parent_mut(),
                RakeNode::Leaf(ref mut leaf) => leaf.parent_mut(),
            }
        }
    }
}

impl<T: Cluster> TVertex<T> for ParentNode<T> {
    fn fix(&mut self) {
        unsafe {
            match *self {
                ParentNode::Compress(mut node) => node.as_mut().fix(),
                ParentNode::Rake(mut leaf) => leaf.as_mut().fix(),
            }
        }
    }

    fn push(&mut self) {
        unsafe {
            match *self {
                ParentNode::Compress(mut node) => node.as_mut().push(),
                ParentNode::Rake(mut leaf) => leaf.as_mut().push(),
            }
        }
    }

    fn reverse(&mut self) {
        unsafe {
            match *self {
                ParentNode::Compress(mut node) => node.as_mut().reverse(),
                ParentNode::Rake(mut leaf) => leaf.as_mut().reverse(),
            }
        }
    }

    fn parent(&self) -> Link<ParentNode<T>> {
        unsafe {
            match *self {
                ParentNode::Compress(node) => node.as_ref().parent(),
                ParentNode::Rake(leaf) => leaf.as_ref().parent(),
            }
        }
    }

    fn parent_mut(&mut self) -> &mut Link<ParentNode<T>> {
        unsafe {
            match self {
                ParentNode::Compress(ref mut node) => node.as_mut().parent_mut(),
                ParentNode::Rake(ref mut leaf) => leaf.as_mut().parent_mut(),
            }
        }
    }
}

impl<T: Cluster> PartialEq for CompNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (CompNode::Node(a), CompNode::Node(b)) => a == b,
            (CompNode::Leaf(a), CompNode::Leaf(b)) => a == b,
            _ => false,
        }
    }
}

impl<T: Cluster> PartialEq for RakeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (RakeNode::Node(a), RakeNode::Node(b)) => a == b,
            (RakeNode::Leaf(a), RakeNode::Leaf(b)) => a == b,
            _ => false,
        }
    }
}

impl<T: Cluster> PartialEq for ParentNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (ParentNode::Compress(a), ParentNode::Compress(b)) => a == b,
            (ParentNode::Rake(a), ParentNode::Rake(b)) => a == b,
            _ => false,
        }
    }
}

impl<T: Cluster> Clone for CompNode<T> {
    fn clone(&self) -> Self {
        match *self {
            CompNode::Node(a) => CompNode::Node(a),
            CompNode::Leaf(a) => CompNode::Leaf(a),
        }
    }
}

impl<T: Cluster> Clone for RakeNode<T> {
    fn clone(&self) -> Self {
        match *self {
            RakeNode::Node(a) => RakeNode::Node(a),
            RakeNode::Leaf(a) => RakeNode::Leaf(a),
        }
    }
}

impl<T: Cluster> Clone for ParentNode<T> {
    fn clone(&self) -> Self {
        match *self {
            ParentNode::Compress(a) => ParentNode::Compress(a),
            ParentNode::Rake(a) => ParentNode::Rake(a),
        }
    }
}

impl<T: Cluster> Copy for CompNode<T> {}
impl<T: Cluster> Copy for RakeNode<T> {}
impl<T: Cluster> Copy for ParentNode<T> {}

fn parent_dir_comp<T: Cluster>(child: CompNode<T>) -> Option<(usize, NonNull<Compress<T>>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Compress(p)) => {
                if p.as_ref().guard {
                    None
                } else if p.as_ref().child(0) == child {
                    Some((0, p))
                } else if p.as_ref().child(1) == child {
                    Some((1, p))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn parent_dir_comp_guard<T: Cluster>(child: CompNode<T>) -> Option<(usize, NonNull<Compress<T>>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Compress(p)) => {
                if p.as_ref().child(0) == child {
                    Some((0, p))
                } else if p.as_ref().child(1) == child {
                    Some((1, p))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn parent_dir_comp_rake<T: Cluster>(child: CompNode<T>) -> Option<(usize, NonNull<Rake<T>>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Rake(p)) => {
                if p.as_ref().child(0) == RakeNode::Leaf(child) {
                    Some((0, p))
                } else if p.as_ref().child(1) == RakeNode::Leaf(child) {
                    Some((1, p))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn parent_dir_rake<T: Cluster>(child: RakeNode<T>) -> Option<(usize, NonNull<Rake<T>>)> {
    unsafe {
        match child.parent() {
            Some(ParentNode::Rake(p)) => {
                if p.as_ref().child(0) == child {
                    Some((0, p))
                } else if p.as_ref().child(1) == child {
                    Some((1, p))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn rotate_comp<T: Cluster>(mut t: NonNull<Compress<T>>, mut x: NonNull<Compress<T>>, dir: usize) {
    unsafe {
        let y = x.as_ref().parent();
        let par = parent_dir_comp_guard(CompNode::Node(x));
        let rake_par = parent_dir_comp_rake(CompNode::Node(x));

        *x.as_mut().child_mut(dir ^ 1) = t.as_ref().child(dir);
        *t.as_ref().child(dir).parent_mut() = Some(ParentNode::Compress(x));
        *t.as_mut().child_mut(dir) = CompNode::Node(x);
        *x.as_mut().parent_mut() = Some(ParentNode::Compress(t));
        *t.as_mut().parent_mut() = y;

        if let Some((xdir, mut yy)) = par {
            *yy.as_mut().child_mut(xdir) = CompNode::Node(t);

            x.as_mut().fix();
            t.as_mut().fix();

            if !yy.as_ref().guard {
                yy.as_mut().fix();
            }
        } else if let Some((xdir, mut yy)) = rake_par {
            *yy.as_mut().child_mut(xdir) = RakeNode::Leaf(CompNode::Node(t));

            x.as_mut().fix();
            t.as_mut().fix();
            yy.as_mut().fix();
        } else if let Some(ParentNode::Compress(mut yy)) = y {
            *yy.as_mut().rake_mut() = Some(RakeNode::Leaf(CompNode::Node(t)));

            x.as_mut().fix();
            t.as_mut().fix();

            if !yy.as_ref().guard {
                yy.as_mut().fix();
            }
        } else {
            x.as_mut().fix();
            t.as_mut().fix();
        }
    }
}

fn rotate_rake<T: Cluster>(mut t: NonNull<Rake<T>>, mut x: NonNull<Rake<T>>, dir: usize) {
    unsafe {
        let y = x.as_ref().parent();
        let par = parent_dir_rake(RakeNode::Node(x));

        *x.as_mut().child_mut(dir ^ 1) = t.as_ref().child(dir);
        *t.as_ref().child(dir).parent_mut() = Some(ParentNode::Rake(x));
        *t.as_mut().child_mut(dir) = RakeNode::Node(x);
        *x.as_mut().parent_mut() = Some(ParentNode::Rake(t));
        *t.as_mut().parent_mut() = y;

        if let Some((xdir, mut yy)) = par {
            *yy.as_mut().child_mut(xdir) = RakeNode::Node(t);

            x.as_mut().fix();
            t.as_mut().fix();
            yy.as_mut().fix();
        } else if let Some(ParentNode::Compress(mut yy)) = y {
            *yy.as_mut().rake_mut() = Some(RakeNode::Node(t));

            x.as_mut().fix();
            t.as_mut().fix();

            if !yy.as_ref().guard {
                yy.as_mut().fix();
            }
        } else {
            x.as_mut().fix();
            t.as_mut().fix();
        }
    }
}

fn splay_comp<T: Cluster>(mut t: NonNull<Compress<T>>) {
    unsafe {
        t.as_mut().push();

        while let Some((_, mut q)) = parent_dir_comp(CompNode::Node(t)) {
            if let Some((_, mut r)) = parent_dir_comp(CompNode::Node(q)) {
                if let Some(mut rp) = r.as_ref().parent() {
                    rp.push();
                }

                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();

                let qt_dir = parent_dir_comp(CompNode::Node(t)).unwrap().0;
                let rq_dir = parent_dir_comp(CompNode::Node(q)).unwrap().0;

                if rq_dir == qt_dir {
                    rotate_comp(q, r, rq_dir ^ 1);
                    rotate_comp(t, q, qt_dir ^ 1);
                } else {
                    rotate_comp(t, q, qt_dir ^ 1);
                    rotate_comp(t, r, rq_dir ^ 1);
                }
            } else {
                if let Some(mut qp) = q.as_ref().parent() {
                    qp.push();
                }

                q.as_mut().push();
                t.as_mut().push();

                let qt_dir = parent_dir_comp(CompNode::Node(t)).unwrap().0;

                rotate_comp(t, q, qt_dir ^ 1);
            }
        }
    }
}

fn splay_rake<T: Cluster>(mut t: NonNull<Rake<T>>) {
    unsafe {
        t.as_mut().push();
        while let Some((_, mut q)) = parent_dir_rake(RakeNode::Node(t)) {
            if let Some((_, mut r)) = parent_dir_rake(RakeNode::Node(q)) {
                if let Some(mut rp) = r.as_ref().parent() {
                    rp.push();
                }

                r.as_mut().push();
                q.as_mut().push();
                t.as_mut().push();

                let qt_dir = parent_dir_rake(RakeNode::Node(t)).unwrap().0;
                let rq_dir = parent_dir_rake(RakeNode::Node(q)).unwrap().0;

                if rq_dir == qt_dir {
                    rotate_rake(q, r, rq_dir ^ 1);
                    rotate_rake(t, q, qt_dir ^ 1);
                } else {
                    rotate_rake(t, q, qt_dir ^ 1);
                    rotate_rake(t, r, rq_dir ^ 1);
                }
            } else {
                if let Some(mut qp) = q.as_ref().parent() {
                    qp.push();
                }

                q.as_mut().push();
                t.as_mut().push();

                let qt_dir = parent_dir_rake(RakeNode::Node(t)).unwrap().0;

                rotate_rake(t, q, qt_dir ^ 1);
            }
        }
    }
}

fn expose_raw<T: Cluster>(mut node: CompNode<T>) -> CompNode<T> {
    loop {
        if let CompNode::Node(comp) = node {
            splay_comp(comp);
        }

        let mut n = match node.parent() {
            None => break,
            Some(ParentNode::Rake(mut par)) => {
                unsafe {
                    par.as_mut().push();
                }

                splay_rake(par);

                if let Some(ParentNode::Compress(n)) = unsafe { par.as_ref().parent() } {
                    n
                } else {
                    unreachable!()
                }
            }
            Some(ParentNode::Compress(mut n)) => {
                unsafe {
                    n.as_mut().push();
                }

                unsafe {
                    if n.as_ref().guard && parent_dir_comp_guard(node).is_some() {
                        break;
                    }
                }

                n
            }
        };

        splay_comp(n);

        let dir = match parent_dir_comp_guard(CompNode::Node(n)) {
            Some((dir, _)) => dir,
            None => 0,
        };

        if dir == 1 {
            unsafe {
                n.as_ref().child(dir).reverse();
                n.as_ref().child(dir).push();
                node.reverse();
                node.push();
            }
        }

        if let Some((n_dir, mut rake)) = parent_dir_rake(RakeNode::Leaf(node)) {
            unsafe {
                let mut nch = n.as_mut().child(dir);
                nch.push();
                rake.as_mut().push();

                *rake.as_mut().child_mut(n_dir) = RakeNode::Leaf(nch);
                *nch.parent_mut() = Some(ParentNode::Rake(rake));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));

                nch.fix();
                rake.as_mut().fix();
                node.fix();
                n.as_mut().fix();

                splay_rake(rake);
            }
        } else {
            unsafe {
                let mut nch = n.as_mut().child(dir);
                nch.push();

                *n.as_mut().rake_mut() = Some(RakeNode::Leaf(nch));
                *nch.parent_mut() = Some(ParentNode::Compress(n));
                *n.as_mut().child_mut(dir) = node;
                *node.parent_mut() = Some(ParentNode::Compress(n));

                nch.fix();
                node.fix();
                n.as_mut().fix();
            }
        }

        if let CompNode::Leaf(_) = node {
            node = CompNode::Node(n);
        }
    }

    node
}

fn expose<T: Cluster>(ver: Vertex<T>) -> CompNode<T> {
    expose_raw(ver.handle().unwrap())
}

fn soft_expose<T: Cluster>(v: Vertex<T>, u: Vertex<T>) {
    unsafe {
        let mut root = expose(v);

        if v.handle() == u.handle() {
            if root.endpoints(1) == v || root.endpoints(0) == u {
                root.reverse();
                root.push();
            }

            return;
        }

        if let CompNode::Node(mut root) = root {
            root.as_mut().guard = true;
            let mut soot = expose(u);
            root.as_mut().guard = false;

            soot.push();
            root.as_mut().fix();

            if let Some((0, _)) = parent_dir_comp(soot) {
                root.as_mut().reverse();
                root.as_mut().push();
            }
        }
    }
}

fn link<T: Cluster>(v: Vertex<T>, u: Vertex<T>, weight: T) -> NonNull<Edge<T>> {
    unsafe {
        if v.handle().is_none() && u.handle().is_none() {
            Edge::new(v, u, weight)
        } else {
            let nnu = u.handle();
            let nnv = v.handle();
            let mut e = Edge::new(v, u, weight);
            let mut left = match nnu {
                None => CompNode::Leaf(e),
                Some(uu) => {
                    let mut uu = expose_raw(uu);

                    uu.push();

                    if uu.endpoints(1) == u {
                        uu.reverse();
                        uu.push();
                    }

                    if uu.endpoints(0) == u {
                        let mut nu = Compress::new(CompNode::Leaf(e), uu);

                        *e.as_mut().parent_mut() = Some(ParentNode::Compress(nu));
                        e.as_mut().fix();
                        *uu.parent_mut() = Some(ParentNode::Compress(nu));
                        uu.fix();
                        nu.as_mut().fix();

                        CompNode::Node(nu)
                    } else {
                        let mut nu = match uu {
                            CompNode::Node(nu) => nu,
                            _ => unreachable!(),
                        };
                        let mut left_ch = nu.as_ref().child(0);

                        left_ch.push();

                        *nu.as_mut().child_mut(0) = CompNode::Leaf(e);
                        *e.as_mut().parent_mut() = Some(ParentNode::Compress(nu));
                        e.as_mut().fix();

                        let beta = nu.as_ref().rake();
                        let mut rake = match beta {
                            Some(mut b) => {
                                b.push();

                                let rake = Rake::new(b, RakeNode::Leaf(left_ch));

                                *b.parent_mut() = Some(ParentNode::Rake(rake));
                                *left_ch.parent_mut() = Some(ParentNode::Rake(rake));

                                b.fix();
                                left_ch.fix();

                                RakeNode::Node(rake)
                            }
                            None => RakeNode::Leaf(left_ch),
                        };

                        rake.fix();
                        *nu.as_mut().rake_mut() = Some(rake);
                        *rake.parent_mut() = Some(ParentNode::Compress(nu));
                        rake.fix();
                        nu.as_mut().fix();

                        CompNode::Node(nu)
                    }
                }
            };

            match nnv {
                None => {}
                Some(vv) => {
                    let mut vv = expose_raw(vv);

                    vv.push();

                    if vv.endpoints(0) == v {
                        vv.reverse();
                        vv.push();
                    }

                    if vv.endpoints(1) == v {
                        let mut top = Compress::new(vv, left);

                        *vv.parent_mut() = Some(ParentNode::Compress(top));
                        vv.fix();
                        *left.parent_mut() = Some(ParentNode::Compress(top));
                        left.fix();
                        top.as_mut().fix();
                    } else {
                        let mut nv = match vv {
                            CompNode::Node(nv) => nv,
                            _ => unreachable!(),
                        };
                        let mut right_ch = nv.as_ref().child(1);

                        right_ch.reverse();
                        right_ch.push();

                        *nv.as_mut().child_mut(1) = left;
                        *left.parent_mut() = Some(ParentNode::Compress(nv));
                        left.fix();

                        let alpha = nv.as_ref().rake();
                        let mut rake = match alpha {
                            Some(mut a) => {
                                a.push();

                                let mut rake = Rake::new(a, RakeNode::Leaf(right_ch));

                                *a.parent_mut() = Some(ParentNode::Rake(rake));
                                *right_ch.parent_mut() = Some(ParentNode::Rake(rake));
                                a.fix();
                                right_ch.fix();
                                rake.as_mut().fix();

                                RakeNode::Node(rake)
                            }
                            None => RakeNode::Leaf(right_ch),
                        };

                        *nv.as_mut().rake_mut() = Some(rake);
                        *rake.parent_mut() = Some(ParentNode::Compress(nv));
                        rake.fix();
                        nv.as_mut().fix();
                    }
                }
            }

            e
        }
    }
}

fn bring<T: Cluster>(mut root: NonNull<Compress<T>>) {
    unsafe {
        match root.as_ref().rake() {
            None => {
                let mut left = root.as_ref().child(0);
                let _ = Box::from_raw(root.as_ptr());

                *left.parent_mut() = None;
                left.fix();
            }
            Some(RakeNode::Leaf(mut new_right)) => {
                new_right.reverse();
                new_right.push();

                *root.as_mut().child_mut(1) = new_right;
                *new_right.parent_mut() = Some(ParentNode::Compress(root));
                *root.as_mut().rake_mut() = None;

                new_right.fix();
                root.as_mut().fix();
            }
            Some(RakeNode::Node(mut rake)) => {
                rake.as_mut().push();

                while let RakeNode::Node(mut right) = rake.as_ref().child(1) {
                    right.as_mut().push();
                    rake = right;
                }

                root.as_mut().guard = true;
                splay_rake(rake);
                root.as_mut().guard = false;

                let mut new_rake = rake.as_ref().child(0);
                let mut new_right = if let RakeNode::Leaf(right) = rake.as_ref().child(1) {
                    right
                } else {
                    unreachable!()
                };

                let _ = Box::from_raw(rake.as_ptr());

                new_right.reverse();
                new_right.push();

                *root.as_mut().child_mut(1) = new_right;
                *new_right.parent_mut() = Some(ParentNode::Compress(root));

                *root.as_mut().rake_mut() = Some(new_rake);
                *new_rake.parent_mut() = Some(ParentNode::Compress(root));

                new_rake.fix();
                new_right.fix();
                root.as_mut().fix();
            }
        }
    }
}

fn cut<T: Cluster>(v: Vertex<T>, u: Vertex<T>) {
    unsafe {
        soft_expose(v, u);

        let mut root = v.handle().unwrap();
        root.push();

        if let CompNode::Node(root) = root {
            let mut right = root.as_ref().child(1);
            *right.parent_mut() = None;

            right.reverse();
            right.push();

            if let CompNode::Node(right) = right {
                if let CompNode::Leaf(_) = right.as_ref().child(1) {
                    bring(right);
                    bring(root);
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        }
    }
}

fn path_query<T: Cluster>(v: Vertex<T>, u: Vertex<T>) -> T {
    unsafe {
        soft_expose(v, u);

        let mut root = v.handle().unwrap();
        root.push();

        if root.endpoints(0) == v && root.endpoints(1) == u {
            root.fold()
        } else if root.endpoints(0) == v {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                n.as_ref().child(0).fold()
            } else {
                unreachable!()
            }
        } else if root.endpoints(1) == u {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();
                n.as_ref().child(1).fold()
            } else {
                unreachable!()
            }
        } else {
            if let CompNode::Node(mut n) = root {
                n.as_mut().push();

                if let CompNode::Node(mut n2) = n.as_ref().child(1) {
                    n2.as_mut().push();
                    n2.as_ref().child(0).fold()
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
}
