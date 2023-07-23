impl Solution {
    pub fn rotate_the_box(bbox: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = bbox.len();
        let n = bbox[0].len();
        let mut count = 0;
        let mut ret = vec![vec!['.'; m]; n];

        for i in 0..m {
            for j in 0..n {
                if bbox[i][j] == '#' {
                    count += 1;
                } else if bbox[i][j] == '*' {
                    ret[j][m - i - 1] = '*';

                    for k in 0..count {
                        ret[j - k - 1][m - i - 1] = '#';
                    }
                    count = 0;
                }
            }

            for k in 0..count {
                ret[n - k - 1][m - i - 1] = '#';
            }
            count = 0;
        }

        ret
    }
}
