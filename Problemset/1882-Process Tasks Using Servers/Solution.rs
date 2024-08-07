use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut free = servers
            .iter()
            .enumerate()
            .map(|(i, &w)| (-w, -(i as i32)))
            .collect::<BinaryHeap<_>>();
        let mut busy: BinaryHeap<(i64, i32)> = BinaryHeap::new();
        let mut time = 0;
        let mut ans = vec![0; tasks.len()];

        for j in 0..tasks.len() {
            time = time.max(j as i64);
            if free.is_empty() {
                time = time.max(-busy.peek().unwrap().0);
            }

            while -busy.peek().unwrap_or(&(i64::MIN + 1, 0)).0 <= time {
                let i = busy.pop().unwrap().1;
                free.push((-servers[i as usize], -i));
            }

            let i = -free.pop().unwrap().1;
            busy.push((-time - tasks[j] as i64, i));
            ans[j] = i;
        }

        ans
    }
}
