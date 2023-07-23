impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut scores = [0; 8];

        for (i, mv) in moves.iter().enumerate() {
            let player = i as i32 % 2 * 2 - 1;
            scores[mv[0] as usize] += player;
            scores[mv[1] as usize + 3] += player;
            if *mv == vec![0, 0] || *mv == vec![1, 1] || *mv == vec![2, 2] {
                scores[6] += player;
            }
            if *mv == vec![0, 2] || *mv == vec![1, 1] || *mv == vec![2, 0] {
                scores[7] += player;
            }
        }

        if scores.contains(&-3) {
            "A".to_string()
        } else if scores.contains(&3) {
            "B".to_string()
        } else if moves.len() == 9 {
            "Draw".to_string()
        } else {
            "Pending".to_string()
        }
    }
}
