impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut to_time = vec![vec![]; n as usize + 1];
        let mut stack = vec![k as usize];
        let mut min_time = vec![std::i32::MAX; n as usize + 1];

        min_time[k as usize] = 0;

        for item in times {
            to_time[item[0] as usize].push((item[1] as usize, item[2]));
        }

        while let Some(from) = stack.pop() {
            for i in 0..to_time[from].len() {
                let (to, time) = to_time[from][i];

                if min_time[to] > min_time[from] + time {
                    min_time[to] = min_time[from] + time;
                    stack.push(to);
                }
            }
        }

        match min_time.into_iter().skip(1).max() {
            Some(std::i32::MAX) => -1,
            Some(t) => t,
            _ => -1,
        }
    }
}
