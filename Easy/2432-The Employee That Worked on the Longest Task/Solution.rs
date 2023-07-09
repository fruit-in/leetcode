impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut start_time = 0;
        let mut max_time = 0;
        let mut ret = 0;

        for i in 0..logs.len() {
            let time = logs[i][1] - start_time;

            if time > max_time || (time == max_time && logs[i][0] < ret) {
                max_time = time;
                ret = logs[i][0];
            }

            start_time = logs[i][1];
        }

        ret
    }
}
