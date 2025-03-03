use io::Write;
use std::{io, str};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const DASH_DECREASE_PERIOD: i64 = 2;
const ACCEL_BUFF_DURATION: i64 = 600;
const GRAVITY_WEAK_DURATION: i64 = 600;
const ITEM_MOTION_BOMB_DURATION: i64 = 100;
const ITEM_MOTION_SHIELD_DURATION: i64 = 30;
const ITEM_MOTION_POTION_DURATION: i64 = 150;

#[derive(Default)]
struct Player {
    pos_x: i64,
    pos_y: i64,
    base_x: i64,
    vel_y: i64,
    jump: i64,
    dash: i64,
    quick: i64,
    accel: i64,
    weak: i64,
    maxdrop: i64,
    motion: i64,
    spin: bool,
    is_vel_y_depend_on_vel_x: bool,
    used_spin_attack: bool,
}

#[derive(Default)]
struct Status {
    remain_dash_decrease: i64,
    remain_accel: i64,
    remain_gravity_weak: i64,
    remain_item_motion: i64,
}

fn update_statuses(player: &mut Player, status: &mut Status) {
    if player.dash > 0 {
        status.remain_dash_decrease -= 1;

        if status.remain_dash_decrease == 0 {
            player.dash -= 1;

            if player.dash > 0 {
                status.remain_dash_decrease = DASH_DECREASE_PERIOD;
            }
        }

        if player.dash == 0 {
            player.is_vel_y_depend_on_vel_x = false;
        }
    }

    if status.remain_accel > 0 {
        status.remain_accel -= 1;

        if status.remain_accel == 0 {
            player.accel = 0;
        }
    }

    if status.remain_gravity_weak > 0 {
        status.remain_gravity_weak -= 1;

        if status.remain_gravity_weak == 0 {
            player.weak = 0;
        }
    }

    if status.remain_item_motion > 0 {
        status.remain_item_motion -= 1;

        if status.remain_item_motion == 0 {
            player.motion = 0;
        }
    }
}

fn process_command(command: &str, player: &mut Player, status: &mut Status, maxdrop: i64) {
    match command {
        "d" => {
            if player.spin {
                player.spin = false;
            }

            if player.quick > 0 {
                player.quick = 0;
            }

            player.dash = 5;
            player.vel_y = 0;
            status.remain_dash_decrease = DASH_DECREASE_PERIOD;
        }
        "aj" => {
            if player.spin {
                player.spin = false;
            }

            player.vel_y = player.jump;
        }
        "qd" => {
            if player.spin {
                player.spin = false;
            }

            player.quick = 1;
            player.vel_y = -maxdrop;
        }
        "ra" => {
            if !player.used_spin_attack {
                player.spin = true;
                player.used_spin_attack = true;
            }
        }
        "hda" => {
            if player.spin {
                player.spin = false;
            }

            if player.quick > 0 {
                player.quick = 0;
            }

            player.dash = 5;
            player.vel_y = 0;
            status.remain_dash_decrease = DASH_DECREASE_PERIOD;
        }
        "ruda" => {
            if player.spin {
                player.spin = false;
            }

            if player.quick > 0 {
                player.quick = 0;
            }

            player.dash = 5;
            player.is_vel_y_depend_on_vel_x = true;
            status.remain_dash_decrease = DASH_DECREASE_PERIOD;
        }
        "acc" => {
            if player.spin {
                player.spin = false;
            }

            player.accel = 1;
            status.remain_accel = ACCEL_BUFF_DURATION;
        }
        "gw" => {
            if player.spin {
                player.spin = false;
            }

            player.weak = 1;
            status.remain_gravity_weak = GRAVITY_WEAK_DURATION;
        }
        "bomb" => {
            if player.spin {
                player.spin = false;
            }

            if player.quick > 0 {
                player.quick = 0;
            }

            player.vel_y = 0;
            player.motion = 1;
            status.remain_item_motion = ITEM_MOTION_BOMB_DURATION;
        }
        "shield" => {
            if player.spin {
                player.spin = false;
            }

            if player.quick > 0 {
                player.quick = 0;
            }

            player.vel_y = 0;
            player.motion = 1;
            status.remain_item_motion = ITEM_MOTION_SHIELD_DURATION;
        }
        "potion" => {
            if player.spin {
                player.spin = false;
            }

            if player.quick > 0 {
                player.quick = 0;
            }

            player.vel_y = 0;
            player.motion = 1;
            status.remain_item_motion = ITEM_MOTION_POTION_DURATION;
        }
        "none" => {
            // Do nothing
        }
        _ => unreachable!(),
    }
}

fn simulate_fall(player: &mut Player, base_g: i64) {
    let frame_remain = (player.vel_y + player.maxdrop - 1) / base_g + 1;
    let vel_x = (1 + player.dash + 5 * player.quick)
        * (1 + player.accel)
        * player.base_x
        * (1 - player.motion);

    if player.pos_y + (2 * player.vel_y - (frame_remain - 1) * base_g) * frame_remain / 2 >= 0 {
        player.pos_y += (2 * player.vel_y - (frame_remain - 1) * base_g) * frame_remain / 2;
        player.pos_x += vel_x * frame_remain;
        player.vel_y = -player.maxdrop;

        let additional_frames = player.pos_y / player.maxdrop
            + if player.pos_y % player.maxdrop == 0 {
                0
            } else {
                1
            };

        player.pos_x += vel_x * additional_frames;
        player.pos_y = 0;
    } else {
        let mut left = 0;
        let mut right = frame_remain;

        while left <= right {
            let mid = (left + right) / 2;

            if player.pos_y + (2 * player.vel_y - (mid - 1) * base_g) * mid / 2 > 0 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        player.pos_x += vel_x * left;
        player.pos_y = 0;
        player.vel_y -= base_g * left;
    }

    if player.spin {
        player.vel_y *= -1;
        player.spin = false;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (base_x, jump, maxdrop) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (base_g, init_y) = (scan.token::<i64>(), scan.token::<i64>());

    let mut player = Player::default();
    let mut status = Status::default();

    player.base_x = base_x;
    player.jump = jump;
    player.maxdrop = maxdrop;
    player.pos_y = init_y;
    player.vel_y = jump;

    let n = scan.token::<usize>();
    let mut commands = Vec::with_capacity(n);

    for _ in 0..n {
        commands.push(scan.token::<String>());
    }

    let mut idx_command = 0;

    while idx_command < n
        || !(player.dash == 0
            && player.accel == 0
            && player.weak == 0
            && player.motion == 0
            && !player.is_vel_y_depend_on_vel_x)
    {
        // Step 1
        if player.pos_y == 0 && !player.spin {
            writeln!(out, "{}", player.pos_x).unwrap();
            return;
        }

        // Step 2
        if player.pos_y == 0 && player.spin {
            player.vel_y *= -1;
            player.spin = false;
        }

        update_statuses(&mut player, &mut status);

        // Step 3
        let command = if idx_command < n {
            &commands[idx_command]
        } else {
            "none"
        };

        if player.dash == 0 && status.remain_item_motion == 0 {
            process_command(command, &mut player, &mut status, maxdrop);
        }

        idx_command += 1;

        // Step 4
        let vel_x = (1 + player.dash + 5 * player.quick)
            * (1 + player.accel)
            * player.base_x
            * (1 - player.motion);
        player.pos_x += vel_x;

        // Step 5
        if player.is_vel_y_depend_on_vel_x {
            player.vel_y = vel_x;
        }

        player.pos_y = (player.pos_y + player.vel_y).max(0);

        // Step 6
        if player.dash == 0 && player.motion == 0 {
            let g = base_g / (1 + player.weak);
            player.vel_y = (-player.maxdrop).max(player.vel_y - g);
        }
    }

    if player.spin {
        simulate_fall(&mut player, base_g);
    }

    simulate_fall(&mut player, base_g);

    writeln!(out, "{}", player.pos_x).unwrap();
}
