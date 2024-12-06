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

    let moves = generate_moves(boards, side);

    if side == "white" {
        let mut eval = i32::MIN;

        for m in moves {
            let mut new_boards = *boards;

            move_piece(&mut new_boards, m);
        }
    } else {
        let mut eval = i32::MAX;

        for m in moves {
            let mut new_boards = *boards;

            move_piece(&mut new_boards, m);
        }
    }

    0
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
                writeln!(stdout, "id name Shrine").unwrap();
                writeln!(stdout, "id author Nate Davis").unwrap();
                writeln!(stdout, "uciok").unwrap();
            }
            "isready" => {
                writeln!(stdout, "readyok").unwrap();
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
                let multiplier = if side == "white" { 1 } else { -1 };

                let best_move = moves
                    .into_iter()
                    .max_by_key(|&m| {
                        let mut new_boards = boards.clone();
                        move_piece(&mut new_boards, m);
                        multiplier * evaluate(&new_boards)
                    })
                    .unwrap();

                let from = index_to_square(best_move.0);
                let to = index_to_square(best_move.1);

                move_piece(&mut boards, best_move);
                writeln!(stdout, "bestmove {}{}", from, to).unwrap();
            }
            "quit" => break,
            _ => writeln!(stdout, "unknown {}", parts[0]).unwrap(),
        }
    }
}
