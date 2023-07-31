use std::{io::Write, isize, usize};

const FIELD_WIDTH: usize = 7;
const FIELD_HEIGHT: usize = 6;
const CONNECT: usize = 4;
const PLAYERS: usize = 2;

type Field = [[usize; FIELD_HEIGHT]; FIELD_WIDTH];
type Players = Vec<String>;

fn main() {
    let mut field = [[0; FIELD_HEIGHT]; FIELD_WIDTH];
    let choose_names = input_bool("Do you want to choose custom names for your players? ");
    let players: Players = (0..PLAYERS)
        .map(|id| {
            if choose_names {
                input(format!("choose a name for Player {}: ", id + 1).as_str())
            } else {
                format!("Player {}", id + 1)
            }
        })
        .collect::<Vec<String>>();

    let mut won = 0;
    let mut current_player = 0;
    while won == 0 {
        let player_name = &players[current_player];
        println!("{player_name}'s turn!");
        print_field(&field);
        let position = input_usize(
            format!("{player_name}, what column do you choose?").as_str(),
            FIELD_WIDTH,
        ) - 1;

        let mut n_pos = (position, FIELD_HEIGHT - 1);
        for i in FIELD_HEIGHT..=0 {
            if field[position][i] == 0 {
                n_pos.1 = i;
                break;
            }
        }
        field[n_pos.0][n_pos.1] = current_player + 1;

        won = find_winner(&mut field, n_pos, current_player + 1);

        current_player = (current_player + 1) % PLAYERS;
    }
    println!("{} WON!", players[won - 1]);
}

fn print_field(field: &Field) {
    for y in 0..FIELD_HEIGHT {
        for x in 0..FIELD_WIDTH {
            print!("|{}", field[x][y]);
        }
        println!("|");
    }
    for i in 0..FIELD_WIDTH {
        print!("|{}", i + 1);
    }
    println!("|");
}

fn input(msg: &str) -> String {
    let mut line = String::new();
    print!("{msg}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim().into()
}

fn input_bool(msg: &str) -> bool {
    fn try_bool_inp(msg: &str, bool: &mut bool) -> bool {
        match input(msg).as_str() {
            "Y" | "y" | "yes" | "Yes" => {
                *bool = true;
                return true;
            }
            "N" | "n" | "no" | "No" => {
                *bool = false;
                return true;
            }
            _ => return false,
        }
    }
    let mut res = false;
    while !try_bool_inp(msg, &mut res) {
        println!("enter either yes or no");
    }
    res
}

fn input_usize(msg: &str, upper_bound: usize) -> usize {
    let try_usize_inp = |msg: &str, res_num: &mut usize| -> bool {
        match input(msg).parse() {
            Ok(v) => {
                if v > upper_bound {
                    return false;
                }
                *res_num = v;
                return true;
            }
            Err(_) => {
                return false;
            }
        }
    };
    let mut res_num = 0;
    while !try_usize_inp(msg, &mut res_num) {
        println!("enter a positive integer (max: {upper_bound})");
    }
    res_num
}

fn find_winner(field: &mut Field, changed: (usize, usize), current_player: usize) -> usize {
    const EITHER_DIR: usize = CONNECT / 2 + 1;
    let mut counts = [0; 4];
    for i in 0..CONNECT * 2 - 1 {
        if let Some(_) = counts.iter().find(|x| **x >= 4) {
            return current_player;
        }

        let d = i as isize - EITHER_DIR as isize;
        let d_inv = -d;
        let x_i = changed.0 as isize + d;
        let y_i = changed.1 as isize + d;
        let y_inv_i = changed.1 as isize + d_inv;

        let x = x_i as usize;
        let y = y_i as usize;
        let y_inv = y_inv_i as usize;

        let x_ok = changed.0 as isize + d >= 0 && changed.0 as isize + d < FIELD_WIDTH as isize;
        let y_ok = changed.1 as isize + d >= 0 && changed.1 as isize + d < FIELD_HEIGHT as isize;
        let y_inv_ok =
            changed.1 as isize + d_inv >= 0 && changed.1 as isize + d_inv < FIELD_HEIGHT as isize;

        if x_ok && field[x][changed.1] == current_player {
            counts[0] += 1;
        }
        if x_ok && y_ok && field[x][y] == current_player {
            counts[1] += 1;
        }
        if y_ok && field[changed.0][y] == current_player {
            counts[2] += 1;
        }
        if x_ok && y_inv_ok && field[x][y_inv] == current_player {
            counts[3] += 1;
        }
    }
    0
}
