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

fn process_game(nums: &Vec<i32>, mut left: usize, mut right: usize) -> bool {
    let mut num_ones = 0;

    for i in left..=right {
        if nums[i] == 1 {
            num_ones += 1;
        }
    }

    // If the number of ones remains "2", the first player can't win.
    // Because XOR operation filps 0 and 1, original -> fliped -> original.
    if (num_ones & 0b10) != 0 {
        return false;
    }

    // The only sequence that the first player can defend is S + aabbccdd... + inv(S)
    // Calculate left/right in case of S and inv(S)
    while left < right && nums[left] == nums[right] {
        left += 1;
        right -= 1;
    }

    // Check aabbccdd...
    let mut i = left;

    while i <= right {
        if (nums[i] ^ nums[i + 1]) != 0 {
            return false;
        }

        i += 2;
    }

    true
}

fn main() {
    let t = input_integers()[0];

    for _ in 0..t {
        let n = input_integers()[0] as usize;
        let mut nums = input_integers();
        let mut sum = 0;

        for j in 0..n {
            sum ^= nums[j];
        }

        // If sum is 0, the game should be "Draw".
        if sum == 0 {
            println!("Draw");
            continue;
        }

        // The number of integers is 1, the game should be "First".
        if n == 1 {
            println!("First");
            continue;
        }

        // The number of integers is even, the game should be "First".
        if n % 2 == 0 {
            println!("First");
            continue;
        }

        let mut msb = 0;

        for j in (0..31).rev() {
            if ((sum >> j) & 1) != 0 {
                msb = j;
                break;
            }
        }

        // Preprocess that each number leaves most significant "1".
        // NOTE: 1 <= X <= 1'000'000'000 ~= 2^30
        for j in 0..n {
            nums[j] = (nums[j] >> msb) & 1;
        }

        // Case 1: The first player selects the beginning of the sequence.
        // NOTE: The first player should select "1" at first turn.
        if nums[0] == 1 && process_game(&nums, 1, n - 1) {
            println!("First");
            continue;
        }

        // Case 2: The first player selects the end of the sequence.
        // NOTE: The first player should select "1" at first turn.
        if nums[n - 1] == 1 && process_game(&nums, 0, n - 2) {
            println!("First");
            continue;
        }

        println!("Second");
    }
}
