use std::collections::HashMap;

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let space = space as i64;
        let mut available_day = HashMap::new();
        let mut ret = 0;

        for i in 0..tasks.len() {
            ret = (ret + 1).max(*available_day.get(&tasks[i]).unwrap_or(&0));
            available_day.insert(tasks[i], ret + space + 1);
        }

        ret
    }
}
