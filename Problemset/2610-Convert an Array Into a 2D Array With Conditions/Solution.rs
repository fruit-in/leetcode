impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut prev = 0;
        let mut count = 0;
        let mut ret = vec![];

        nums.sort_unstable();

        for &x in &nums {
            if x != prev {
                prev = x;
                count = 0;
            }
            count += 1;
            if count > ret.len() {
                ret.push(vec![]);
            }
            ret[count - 1].push(x);
        }

        ret
    }
}
