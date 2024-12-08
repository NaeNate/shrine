mod evaluate;
mod generate_moves;
mod move_piece;

use evaluate::evaluate;
use generate_moves::generate_moves;
use move_piece::move_piece;
use std::{
    i32,
    io::{stdin, stdout, Write},
};

fn square_to_index(square: &str) -> u8 {
    let file = square.as_bytes()[0] - b'a';
    let rank = square.as_bytes()[1] - b'1';

    rank * 8 + file
}

fn index_to_square(index: u8) -> String {
    let file = (index % 8) + b'a';
    let rank = (index / 8) + b'1';

    format!("{}{}", file as char, rank as char)
}

fn minimax(boards: &[u64; 12], side: &str, depth: u8) -> i32 {
    if depth == 0 {
        return evaluate(boards);
    }

    let (mut eval, other) = if side == "white" {
        (i32::MIN, "black")
    } else {
        (i32::MAX, "white")
    };

    for m in generate_moves(boards, side) {
        let mut new_boards = *boards;
        move_piece(&mut new_boards, m);

        let new_eval = minimax(&new_boards, other, depth - 1);

        if side == "white" {
            eval = eval.max(new_eval)
        } else {
            eval = eval.min(new_eval)
        }
    }

    eval
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    let mut setup = false;
    let mut boards: [u64; 12] = [0; 12];
    let mut side = "white";

    for line in stdin.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[0] {
            "uci" => {
                println!("id name Shrine");
                println!("id author Nate Davis");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "ucinewgame" => {}
            "position" => {
                if setup {
                    let last = parts.last().unwrap();

                    let from = square_to_index(&last[0..2]);
                    let to = square_to_index(&last[2..4]);
                    let promotion = last.chars().nth(4);

                    move_piece(&mut boards, (from, to, promotion));
                } else {
                    if parts[1] == "startpos" {
                        boards = [
                            0b11111111 << 8,
                            (1 << 1) | (1 << 6),
                            (1 << 2) | (1 << 5),
                            (1 << 0) | (1 << 7),
                            (1 << 3),
                            (1 << 4),
                            0b11111111 << 48,
                            (1 << 57) | (1 << 62),
                            (1 << 58) | (1 << 61),
                            (1 << 56) | (1 << 63),
                            (1 << 59),
                            (1 << 60),
                        ]
                    }

                    if parts.contains(&"moves") {
                        let last = parts.last().unwrap();

                        let from = square_to_index(&last[0..2]);
                        let to = square_to_index(&last[2..4]);
                        let promotion = last.chars().nth(4);

                        move_piece(&mut boards, (from, to, promotion));

                        side = "black"
                    }

                    setup = true
                }
            }
            "go" => {
                let moves = generate_moves(&boards, side);
                let depth = 5;

                let mut best_move = moves[0];
                let (mut best_eval, other) = match side {
                    "white" => (i32::MIN, "black"),
                    "black" => (i32::MAX, "white"),
                    _ => panic!(),
                };

                for m in moves {
                    let mut new_boards = boards;
                    move_piece(&mut new_boards, m);

                    let eval = minimax(&new_boards, other, depth - 1);

                    if (side == "white" && eval > best_eval)
                        || (side == "black" && eval < best_eval)
                    {
                        best_eval = eval;
                        best_move = m;
                    }
                }

                let from = index_to_square(best_move.0);
                let to = index_to_square(best_move.1);
                let promotion = best_move.2.unwrap_or(' ');

                move_piece(&mut boards, best_move);
                println!("bestmove {}{}{}", from, to, promotion);
            }
            "quit" => break,
            _ => println!("unknown {}", parts[0]),
        }
    }
}
