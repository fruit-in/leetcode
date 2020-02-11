impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let dir = vec![(-1, 0, king[0]),
                       (0, -1, king[1]),
                       (1, 0, 7 - king[0]),
                       (0, 1, 7 - king[1]),
                       (-1, 1, king[0].min(7 - king[1])),
                       (1, -1, king[1].min(7 - king[0])),
                       (-1, -1, king[0].min(king[1])),
                       (1, 1, 7 - king[0].max(king[1]))];

        for (a, b, c) in dir {
            for i in 1..=c {
                if queens.contains(&vec![king[0] + a * i, king[1] + b * i]) {
                    ret.push(vec![king[0] + a * i, king[1] + b * i]);
                    break;
                }
            }
        }

        ret
    }
}
