impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        let mut energy = 0;
        let mut ret = 0;

        tasks.sort_unstable_by_key(|t| t[0] - t[1]);

        for task in &tasks {
            if energy < task[1] {
                ret += task[1] - energy;
                energy = task[1];
            }
            energy -= task[0];
        }

        ret
    }
}
