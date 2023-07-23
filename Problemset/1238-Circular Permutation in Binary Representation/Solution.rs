impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut x = 1;
        let mut start_index = 0;
        let mut ret = vec![0];

        for _ in 0..n {
            let rev = ret.iter().rev().map(|&num| num + x).collect::<Vec<i32>>();
            for i in 0..rev.len() {
                ret.push(rev[i]);
                if rev[i] == start {
                    start_index = ret.len() - 1;
                }
            }
            x *= 2;
        }

        ret.rotate_left(start_index);

        ret
    }
}
