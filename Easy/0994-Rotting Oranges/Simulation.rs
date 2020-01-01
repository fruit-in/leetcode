impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut prev = grid;

        for minute in 0..59 {
            let mut no_fresh = true;
            let mut new = prev.clone();

            for j in 0..prev.len() {
                for k in 0..prev[0].len() {
                    match prev[j][k] {
                        1 => no_fresh = false,
                        2 => {
                            if j > 0 && prev[j - 1][k] == 1 {
                                new[j - 1][k] = 2;
                            }
                            if k > 0 && prev[j][k - 1] == 1 {
                                new[j][k - 1] = 2;
                            }
                            if j < prev.len() - 1 && prev[j + 1][k] == 1 {
                                new[j + 1][k] = 2;
                            }
                            if k < prev[0].len() - 1 && prev[j][k + 1] == 1 {
                                new[j][k + 1] = 2;
                            }
                        },
                        _ => (),
                    };
                }
            }

            if no_fresh {
                return minute;
            }

            prev = new;
        }

        -1
    }
}
