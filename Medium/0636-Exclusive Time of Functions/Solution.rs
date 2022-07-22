impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![0; n as usize];

        for log in logs {
            let log = log.split(':').collect::<Vec<_>>();
            let function_id = log[0].parse::<usize>().unwrap();
            let timestamp = log[2].parse::<i32>().unwrap();

            if log[1] == "start" {
                if let Some(&(id, time)) = stack.last() {
                    ret[id] += timestamp - time;
                }

                stack.push((function_id, timestamp));
            } else {
                let (_, time) = stack.pop().unwrap();

                ret[function_id] += timestamp - time + 1;

                if let Some((id, _)) = stack.pop() {
                    stack.push((id, timestamp + 1));
                }
            }
        }

        ret
    }
}
