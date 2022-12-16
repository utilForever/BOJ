use io::Write;
use std::{
    collections::HashMap,
    io::{self, BufWriter, StdinLock, StdoutLock},
    str,
};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
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

#[derive(Debug)]
struct Actor {
    pub parent: Option<String>,
    pub childs: Vec<String>,
    pub position: (i64, i64),
    pub size: (i64, i64),
    pub color: i64,
    pub parent_origin: (i64, i64),
    pub anchor_point: (i64, i64),
}

impl Actor {
    fn new() -> Self {
        Self {
            parent: None,
            childs: Vec::new(),
            position: (0, 0),
            size: (0, 0),
            color: 1,
            parent_origin: (0, 0),
            anchor_point: (0, 0),
        }
    }
}

fn new(actors: &mut HashMap<String, Actor>, name: String) {
    actors.insert(name, Actor::new());
}

fn add(actors: &mut HashMap<String, Actor>, name1: String, name2: String) {
    unparent(actors, name1.clone(), actors[&name1].parent.clone());

    actors.get_mut(&name1).unwrap().parent = Some(name2.clone());
    actors.get_mut(&name2).unwrap().childs.push(name1);
}

fn remove(actors: &mut HashMap<String, Actor>, name1: String, name2: String) {
    if let Some(pos) = actors[&name2].childs.iter().position(|x| *x == name1) {
        actors.get_mut(&name2).unwrap().childs.remove(pos);
        actors.get_mut(&name1).unwrap().parent = None;
    }
}

fn unparent(actors: &mut HashMap<String, Actor>, name: String, parent: Option<String>) {
    if parent.is_none() {
        return;
    }

    let parent = parent.unwrap();

    if let Some(parent_of_name) = &actors[&name].parent {
        if *parent_of_name == parent {
            let pos = actors[&parent]
                .childs
                .iter()
                .position(|x| *x == name)
                .unwrap();

            actors.get_mut(&parent).unwrap().childs.remove(pos);
            actors.get_mut(&name).unwrap().parent = None;
        }
    }
}

fn set_property(
    scan: &mut UnsafeScanner<StdinLock>,
    actors: &mut HashMap<String, Actor>,
    name_actor: String,
    name_property: String,
) {
    match name_property.as_str() {
        "POSITION" => {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            actors.get_mut(&name_actor).unwrap().position = (x, y);
        }
        "SIZE" => {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            actors.get_mut(&name_actor).unwrap().size = (x, y);
        }
        "COLOR" => {
            let v = scan.token::<i64>();
            actors.get_mut(&name_actor).unwrap().color = v;
        }
        "PARENT_ORIGIN" => {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            actors.get_mut(&name_actor).unwrap().parent_origin = (x, y);
        }
        "ANCHOR_POINT" => {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            actors.get_mut(&name_actor).unwrap().anchor_point = (x, y);
        }
        _ => panic!("Unknown property"),
    }
}

fn get_property(
    out: &mut BufWriter<StdoutLock>,
    actors: &mut HashMap<String, Actor>,
    name_actor: String,
    name_property: String,
) {
    match name_property.as_str() {
        "POSITION" => {
            let (x, y) = actors[&name_actor].position;
            writeln!(out, "{x} {y}").unwrap();
        }
        "SIZE" => {
            let (x, y) = actors[&name_actor].size;
            writeln!(out, "{x} {y}").unwrap();
        }
        "COLOR" => {
            writeln!(out, "{}", actors[&name_actor].color).unwrap();
        }
        "PARENT_ORIGIN" => {
            let (x, y) = actors[&name_actor].parent_origin;
            writeln!(out, "{x} {y}").unwrap();
        }
        "ANCHOR_POINT" => {
            let (x, y) = actors[&name_actor].anchor_point;
            writeln!(out, "{x} {y}").unwrap();
        }
        "SCREEN_POSITION" => {
            let mut cur = Some(name_actor.clone());
            let mut temp_actors = Vec::new();

            while cur.is_some() {
                temp_actors.push(cur.as_ref().unwrap().clone());
                cur = actors[cur.as_ref().unwrap()].parent.clone();
            }

            if temp_actors.last().unwrap() != "Window" {
                writeln!(out, "0 0").unwrap();
            } else {
                temp_actors.reverse();

                let mut x = 0;
                let mut y = 0;

                for i in 0..temp_actors.len() - 1 {
                    let actor_cur = temp_actors[i].clone();
                    let actor_next = temp_actors[i + 1].clone();
                    let mut dx = 0;
                    let mut dy = 0;

                    if actors[&actor_next].parent_origin.0 == 1 {
                        dx += actors[&actor_cur].size.0;
                    }

                    if actors[&actor_next].parent_origin.1 == 1 {
                        dy += actors[&actor_cur].size.1;
                    }

                    if actors[&actor_next].anchor_point.0 == 1 {
                        dx -= actors[&actor_next].size.0;
                    }

                    if actors[&actor_next].anchor_point.1 == 1 {
                        dy -= actors[&actor_next].size.1;
                    }

                    dx += actors[&actor_next].position.0;
                    dy += actors[&actor_next].position.1;

                    x += dx;
                    y += dy;
                }

                if actors[&name_actor].anchor_point.0 == 1 {
                    x += actors[&name_actor].size.0;
                }

                if actors[&name_actor].anchor_point.1 == 1 {
                    y += actors[&name_actor].size.1;
                }

                writeln!(out, "{x} {y}").unwrap();
            }
        }
        _ => panic!("Unknown property"),
    }
}

fn draw(
    screen: &mut Vec<Vec<i64>>,
    actors: &HashMap<String, Actor>,
    name_node: String,
    name_parent: Option<String>,
    mut x: i64,
    mut y: i64,
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
) {
    if actors[&name_node].parent_origin.0 == 1 {
        x += actors[name_parent.as_ref().unwrap()].size.0;
    }

    if actors[&name_node].parent_origin.1 == 1 {
        y += actors[name_parent.as_ref().unwrap()].size.1;
    }

    if actors[&name_node].anchor_point.0 == 1 {
        x -= actors[&name_node].size.0;
    }

    if actors[&name_node].anchor_point.1 == 1 {
        y -= actors[&name_node].size.1;
    }

    x += actors[&name_node].position.0;
    y += actors[&name_node].position.1;

    let x_start = x1.max(x.max(0));
    let y_start = y1.max(y.max(0));
    let x_end = x2.min((x + actors[&name_node].size.0).min(screen[0].len() as i64));
    let y_end = y2.min((y + actors[&name_node].size.1).min(screen.len() as i64));

    for i in y_start..y_end {
        for j in x_start..x_end {
            screen[i as usize][j as usize] = actors[&name_node].color;
        }
    }

    for child in &actors[&name_node].childs {
        draw(
            screen,
            actors,
            child.clone(),
            Some(name_node.clone()),
            x,
            y,
            x1,
            y1,
            x2,
            y2,
        );
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (w, h) = (scan.token::<usize>(), scan.token::<usize>());
    let q = scan.token::<i64>();
    let mut screen = vec![vec![0; w]; h];
    let mut actors = HashMap::new();

    actors.insert("Window".to_string(), Actor::new());
    actors.get_mut("Window").unwrap().size = (w as i64, h as i64);
    actors.get_mut("Window").unwrap().color = 0;

    for _ in 0..q {
        let command = scan.token::<String>();

        match command.as_str() {
            "New" => {
                let name = scan.token::<String>();
                new(&mut actors, name);
            }
            "Add" => {
                let (name1, name2) = (scan.token::<String>(), scan.token::<String>());
                add(&mut actors, name2, name1);
            }
            "Remove" => {
                let (name1, name2) = (scan.token::<String>(), scan.token::<String>());
                remove(&mut actors, name2, name1);
            }
            "Unparent" => {
                let name = scan.token::<String>();
                let parent = actors[&name].parent.clone();
                unparent(&mut actors, name.clone(), parent);
            }
            "SetProperty" => {
                let (name_actor, name_property) = (scan.token::<String>(), scan.token::<String>());
                set_property(&mut scan, &mut actors, name_actor.clone(), name_property);
            }
            "GetProperty" => {
                let (name_actor, name_property) = (scan.token::<String>(), scan.token::<String>());
                get_property(&mut out, &mut actors, name_actor, name_property);
            }
            _ => panic!("Unknown command: {}", command),
        }
    }

    draw(
        &mut screen,
        &actors,
        "Window".to_string(),
        None,
        0,
        0,
        0,
        0,
        w as i64,
        h as i64,
    );

    for i in 0..h {
        for j in 0..w {
            write!(out, "{}", screen[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
