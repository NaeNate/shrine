pub fn move_piece(boards: &mut [u64; 12], mv: (u8, u8, Option<char>)) {
    let (from, to, promotion) = mv;

    let from_mask = 1 << from;
    let to_mask = 1 << to;

    for i in 0..12 {
        if boards[i] & from_mask != 0 {
            boards[i] &= !from_mask;
            boards[i] |= to_mask;

            for j in 0..12 {
                if i != j {
                    boards[j] &= !to_mask;
                }
            }

            break;
        }
    }
}
