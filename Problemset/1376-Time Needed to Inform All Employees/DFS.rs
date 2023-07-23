use std::collections::HashMap;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut subordinates = HashMap::new();

        for i in 0..manager.len() {
            subordinates
                .entry(manager[i])
                .or_insert(vec![])
                .push(i as i32);
        }

        Self::dfs(head_id, &subordinates, &inform_time)
    }

    fn dfs(head_id: i32, subordinates: &HashMap<i32, Vec<i32>>, inform_time: &Vec<i32>) -> i32 {
        match subordinates.get(&head_id) {
            Some(subs) => {
                inform_time[head_id as usize]
                    + subs
                        .iter()
                        .map(|&id| Self::dfs(id, subordinates, inform_time))
                        .max()
                        .unwrap()
            }
            None => 0,
        }
    }
}
