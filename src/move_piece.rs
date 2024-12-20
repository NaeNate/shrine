pub fn move_piece(boards: &mut [u64; 12], m: (u8, u8, Option<char>), en_passant: &mut Option<u8>) {
    let (from, to, promotion) = m;

    let from_mask = 1 << from;
    let to_mask = 1 << to;

    *en_passant = None;

    for i in 0..12 {
        if boards[i] & from_mask != 0 {
            if i % 6 == 0 {
                if (to as i8 - from as i8) == 16 {
                    *en_passant = Some(from + 8)
                } else if to as i8 - from as i8 == -16 {
                    *en_passant = Some(from - 8)
                }
            }

            let mut place = i;

            if let Some(prom) = promotion {
                let offset = (i / 6) * 6;

                let piece = match prom {
                    'q' => offset + 4,
                    'r' => offset + 3,
                    'b' => offset + 2,
                    'n' => offset + 1,
                    _ => offset,
                };

                place = piece;
            }

            boards[i] &= !from_mask;
            boards[place] |= to_mask;

            for j in 0..12 {
                if i != j {
                    boards[j] &= !to_mask;
                }
            }

            break;
        }
    }
}
