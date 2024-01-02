impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count = [0; 27];
        let mut cooldown = [0; 26];
        let mut time = 0;

        for &task in &tasks {
            count[(task as usize) - 65] += 1;
        }

        for _ in 0..tasks.len() {
            let mut min_cooldown = i32::MAX;
            let mut next_task = 26;

            for i in 0..26 {
                if count[i] > 0 {
                    min_cooldown = min_cooldown.min(cooldown[i]);
                }
            }
            time = time.max(min_cooldown);

            for i in 0..26 {
                if cooldown[i] <= time && count[i] > count[next_task] {
                    next_task = i;
                }
            }
            count[next_task] -= 1;
            time += 1;
            cooldown[next_task] = time + n;
        }

        time
    }
}
