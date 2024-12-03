pub fn generate_moves(boards: &[u64; 12], side: &str) -> Vec<(u8, u8, Option<char>)> {
    let mut moves = Vec::new();

    let offset = if side == "white" { 0 } else { 6 };

    let friends = boards[offset]
        | boards[offset + 1]
        | boards[offset + 2]
        | boards[offset + 3]
        | boards[offset + 4]
        | boards[offset + 5];

    let enemies = boards[6 - offset]
        | boards[(6 - offset) + 1]
        | boards[(6 - offset) + 2]
        | boards[(6 - offset) + 3]
        | boards[(6 - offset) + 4]
        | boards[(6 - offset) + 5];

    let everyone = friends | enemies;

    let mut pawns = boards[offset];

    while pawns != 0 {
        let index = pawns.trailing_zeros() as u8;

        let direction = if side == "white" { 8 } else { -8 };
        let target = (index as i8 + direction) as u8;

        if (everyone & (1 << target)) == 0 {
            moves.push((index, target, Some('p')));
        }

        pawns &= pawns - 1
    }

    let mut knights = boards[offset + 1];

    while knights != 0 {
        let index = knights.trailing_zeros() as u8;

        for direction in [-17, -15, -10, -6, 6, 10, 15, 17] {
            let target = (index as i8 + direction) as u8;

            if target >= 64 {
                continue;
            }

            if (index as i8 % 8 - target as i8 % 8).abs() > 2 {
                continue;
            }

            if (friends & (1 << target)) != 0 {
                continue;
            }

            moves.push((index, target, Some('n')))
        }

        knights &= knights - 1
    }

    let mut bishops = boards[offset + 2];

    while bishops != 0 {
        let index = bishops.trailing_zeros() as u8;

        for direction in [-9, -7, 7, 9] {
            let mut target = index as i8;

            loop {
                target += direction;

                if target < 0 || target >= 64 {
                    break;
                }

                if (direction == -9 || direction == 7) && index % 8 == 0 {
                    break;
                }

                if (direction == 9 || direction == -7) && index % 8 == 7 {
                    break;
                }

                if (friends & (1 << target)) != 0 {
                    break;
                }

                moves.push((index, target as u8, Some('b')));

                if (enemies & (1 << target)) != 0 {
                    break;
                }
            }
        }

        bishops &= bishops - 1
    }

    let mut rooks = boards[offset + 3];

    while rooks != 0 {
        let index = rooks.trailing_zeros() as u8;

        for direction in [-8, -1, 1, 8] {
            let mut target = index as i8;

            loop {
                target += direction;

                if target < 0 || target >= 64 {
                    break;
                }
                if direction == -1 && index % 8 == 0 {
                    break;
                }

                if direction == 1 && index % 8 == 7 {
                    break;
                }

                if (friends & (1 << target)) != 0 {
                    break;
                }

                moves.push((index, target as u8, Some('r')));

                if (enemies & (1 << target)) != 0 {
                    break;
                }
            }
        }

        rooks &= rooks - 1
    }

    let mut queens = boards[offset + 4];

    while queens != 0 {
        let index = queens.trailing_zeros() as u8;

        for direction in [-9, -8, -7, -1, 1, 7, 8, 9] {
            let mut target = index as i8;

            loop {
                target += direction;

                if target < 0 || target >= 64 {
                    break;
                }

                if (direction == -9 || direction == 7 || direction == -1) && index % 8 == 0 {
                    break;
                }

                if (direction == 9 || direction == -7 || direction == 1) && index % 8 == 7 {
                    break;
                }

                if (friends & (1 << target)) != 0 {
                    break;
                }

                moves.push((index, target as u8, Some('q')));

                if (enemies & (1 << target)) != 0 {
                    break;
                }
            }
        }

        queens &= queens - 1
    }

    let mut kings = boards[offset + 5];

    while kings != 0 {
        let index = kings.trailing_zeros() as u8;

        for direction in [-9, -8, -7, -1, 1, 7, 8, 9] {
            let target = (index as i8 + direction) as u8;

            if target >= 64 {
                continue;
            }

            if (direction == -9 || direction == 7 || direction == -1) && index % 8 == 0 {
                continue;
            }

            if (direction == 9 || direction == -7 || direction == 1) && index % 8 == 7 {
                continue;
            }

            if (friends & (1 << target)) != 0 {
                continue;
            }

            moves.push((index, target, Some('k')))
        }

        kings &= kings - 1
    }

    moves
}
