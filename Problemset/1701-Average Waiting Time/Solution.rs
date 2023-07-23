impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut time = 0u64;
        let mut sum = 0u64;

        for customer in &customers {
            if customer[0] as u64 >= time {
                sum += customer[1] as u64;
                time = (customer[0] + customer[1]) as u64;
            } else {
                sum += time + (customer[1] - customer[0]) as u64;
                time += customer[1] as u64;
            }
        }

        sum as f64 / customers.len() as f64
    }
}
