use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn input_chars() -> Vec<char> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<char> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn preorder(tree: &[[i32; 2]; 26], n: i32, num_node: usize) {
    if n == num_node as i32 || n == -1 {
        return;
    }

    print!("{}", (n as u8 + 'A' as u8) as char);
    preorder(tree, tree[n as usize][0], num_node);
    preorder(tree, tree[n as usize][1], num_node);
}

fn inorder(tree: &[[i32; 2]; 26], n: i32, num_node: usize) {
    if n == num_node as i32 || n == -1 {
        return;
    }

    inorder(tree, tree[n as usize][0], num_node);
    print!("{}", (n as u8 + 'A' as u8) as char);
    inorder(tree, tree[n as usize][1], num_node);
}

fn postorder(tree: &[[i32; 2]; 26], n: i32, num_node: usize) {
    if n == num_node as i32 || n == -1 {
        return;
    }

    postorder(tree, tree[n as usize][0], num_node);
    postorder(tree, tree[n as usize][1], num_node);
    print!("{}", (n as u8 + 'A' as u8) as char);
}

fn main() {
    let n = input_integers()[0] as usize;

    let mut tree = [[0_i32; 2]; 26];

    for _ in 0..n {
        let chars = input_chars();
        let (node, left, right) = (chars[0], chars[1], chars[2]);

        if left == '.' {
            tree[(node as u32 - 'A' as u32) as usize][0] = -1;
        } else {
            tree[(node as u32 - 'A' as u32) as usize][0] = (left as u32 - 'A' as u32) as i32;
        }

        if right == '.' {
            tree[(node as u32 - 'A' as u32) as usize][1] = -1;
        } else {
            tree[(node as u32 - 'A' as u32) as usize][1] = (right as u32 - 'A' as u32) as i32;
        }
    }

    preorder(&tree, 0, n);
    println!();
    inorder(&tree, 0, n);
    println!();
    postorder(&tree, 0, n);
    println!();
}
