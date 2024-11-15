use std::io;

#[derive(PartialEq)]
enum Result {
    Cracked,
    Safe,
    Egg
}

fn main() {
    let mut pos: (usize, usize) = (1, 1);
    let target: (usize, usize) = (3, 3);
    let mut result: Result = Result::Egg;
    print_board(&pos, &target, &result);

    loop {
        let mut movement = String::new();

        io::stdin()
            .read_line(&mut movement)
            .expect("Failed to read line");

        let movement = &movement[0..1];
        println!("you moved: {movement}");

        match movement {
            "l" => {
                if pos.0 == 1 {
                    result = Result::Cracked;
                    print_board(&pos, &target, &result);
                    println!("you failed :<");
                    break
                }
                pos.0 -= 1;
                result = check_success(&pos, &target);
                print_board(&pos, &target, &result);
            },
            "r" => {
                if pos.0 == 3 {
                    result = Result::Cracked;
                    print_board(&pos, &target, &result);
                    println!("you failed :<");
                    break
                }
                pos.0 += 1;
                result = check_success(&pos, &target);
                print_board(&pos, &target, &result);
            },
            "u" => {
                if pos.1 == 1 {
                    result = Result::Cracked;
                    print_board(&pos, &target, &result);
                    println!("you failed :<");
                    break
                }
                pos.1 -= 1;
                result = check_success(&pos, &target);
                print_board(&pos, &target, &result);
            },
            "d" => {
                if pos.1 == 3 {
                    result = Result::Cracked;
                    print_board(&pos, &target, &result);
                    println!("you failed :<");
                    break
                }
                pos.1 += 1;
                result = check_success(&pos, &target);
                print_board(&pos, &target, &result);
            },
            "q" => {
                println!("quit");
                break;
            },
            _ => continue
        }

        if pos.0 == target.0 && pos.1 == target.1 {
            println!("yay!!!");
            break
        }
    }
}

fn check_success(pos: &(usize, usize), target: &(usize, usize)) -> Result {
    if pos.0 == target.0 && pos.1 == target.1 {
        Result::Safe
    }
    else {
        Result::Egg
    }
}

fn print_board(pos: &(usize, usize), target: &(usize, usize), result: &Result) {
    let area = [['🧱'; 5]; 5];
    for r_idx in 0..area.len() {
        for c_idx in 0..area[r_idx].len() {
            if r_idx == 0 ||
            r_idx == area.len() - 1 ||
            c_idx == 0 ||
            c_idx == area[r_idx].len() - 1 {
                let item = area[r_idx][c_idx];
                print!("{item}");
            }
            else {
                let (x, y) = pos;
                if *y == r_idx && *x == c_idx {
                    if *result == Result::Safe {
                        print!("🪺")
                    }
                    else if *result == Result::Cracked {
                        print!("💀")
                    }
                    else {
                        print!("🥚")
                    }
                } else {
                    let (t_x, t_y) = target;
                    if *t_y == r_idx && *t_x == c_idx {
                        print!("🪹")
                    }
                    else {
                        print!("⬛")
                    }
                }
            }
        }
        println!();
    }
}