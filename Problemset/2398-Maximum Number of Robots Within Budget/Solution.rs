use std::collections::VecDeque;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut k = 0;
        let mut deque = VecDeque::new();
        let mut max_charge = 0;
        let mut running_sum = 0;
        let mut total_cost = 0;
        let mut ret = 0;

        for i in 0..charge_times.len() {
            k += 1;
            while deque.back().unwrap_or(&(0, i64::MAX)).1 <= charge_times[i] as i64 {
                deque.pop_back();
            }
            deque.push_back((i, charge_times[i] as i64));
            max_charge = deque.front().unwrap().1;
            running_sum += running_costs[i] as i64;
            total_cost = max_charge + k as i64 * running_sum;

            while total_cost > budget {
                k -= 1;
                if deque.front().unwrap().0 < i - k + 1 {
                    deque.pop_front();
                }
                max_charge = deque.front().unwrap_or(&(0, 0)).1;
                running_sum -= running_costs[i - k] as i64;
                total_cost = max_charge + k as i64 * running_sum;
            }

            ret = ret.max(k as i32);
        }

        ret
    }
}
