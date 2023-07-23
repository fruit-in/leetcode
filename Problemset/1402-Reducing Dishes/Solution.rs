impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        let mut sum = 0;
        let mut ret = 0;
        satisfaction.sort_unstable();

        for i in (0..satisfaction.len()).rev() {
            sum += satisfaction[i];
            if sum < 0 {
                break;
            }
            ret += sum;
        }

        ret
    }
}
