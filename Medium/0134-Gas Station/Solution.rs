impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut tank = 0;

        for i in 0..(2 * gas.len()) {
            let j = i % gas.len();

            tank += gas[j] - cost[j];

            if tank >= 0 && (j + 1) % gas.len() == start {
                return start as i32;
            } else if tank < 0 {
                start = j + 1;
                tank = 0;
            }
        }

        -1
    }
}
