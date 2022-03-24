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

fn convert_pre_to_post(preorder: &Vec<i32>, left: usize, right: usize) {
    if left > right {
        return;
    }
    
    let cur_node = left;
    let left_node = left + 1;
    let mut right_node = left + 1;

    while right_node < preorder.len() && preorder[right_node] < preorder[cur_node] {
        right_node += 1;
    }

    if left <= right {
        convert_pre_to_post(preorder, left_node, right_node - 1);
        convert_pre_to_post(preorder, right_node, right);
    }

    println!("{}", preorder[cur_node]);
}

fn main() {
    let mut preorder = Vec::new();

    loop {
        let nums = input_integers();

        if nums.is_empty() {
            break;
        }

        preorder.push(nums[0]);
    }

    convert_pre_to_post(&preorder, 0, preorder.len() - 1);
}
