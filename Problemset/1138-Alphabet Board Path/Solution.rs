impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut s = (0, 0);
        let mut ret = String::new();

        for c in target.bytes() {
            let t = ((c - b'a') / 5, (c - b'a') % 5);
            ret = ret + &Self::path_to(s, t) + "!";
            s = t;
        }

        ret
    }

    fn path_to(start: (u8, u8), target: (u8, u8)) -> String {
        if start == target {
            String::new()
        } else if start == (5, 0) {
            "U".to_string() + &Self::path_to((4, 0), target)
        } else if target == (5, 0) {
            Self::path_to(start, (4, 0)) + "D"
        } else {
            let mut moves = String::new();
            if start.0 > target.0 {
                moves += &"U".repeat((start.0 - target.0) as usize)
            } else {
                moves += &"D".repeat((target.0 - start.0) as usize)
            }
            if start.1 > target.1 {
                moves += &"L".repeat((start.1 - target.1) as usize);
            } else {
                moves += &"R".repeat((target.1 - start.1) as usize);
            }
            moves
        }
    }
}
