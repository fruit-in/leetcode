use std::collections::HashMap;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut indegrees = vec![0; num_courses as usize];
        let mut follows = HashMap::new();
        let mut courses = vec![];

        for prerequisite in prerequisites {
            indegrees[prerequisite[0] as usize] += 1;
            follows
                .entry(prerequisite[1] as usize)
                .or_insert(vec![])
                .push(prerequisite[0] as usize);
        }

        for b in 0..num_courses as usize {
            if indegrees[b] == 0 {
                courses.push(b);
            }
        }

        while let Some(b) = courses.pop() {
            for a in follows.remove(&b).unwrap_or(vec![]) {
                indegrees[a] -= 1;
                if indegrees[a] == 0 {
                    courses.push(a);
                }
            }
        }

        indegrees.iter().all(|&x| x == 0)
    }
}
