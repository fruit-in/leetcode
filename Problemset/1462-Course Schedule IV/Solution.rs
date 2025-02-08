impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        use std::collections::HashSet;

        let num_courses = num_courses as usize;
        let mut prerequisites_set = HashSet::new();
        let mut is_prerequisite = vec![vec![false; num_courses]; num_courses];
        let mut indegree = vec![0; num_courses];
        let mut stack = vec![];

        for prerequisite in &prerequisites {
            let (a, b) = (prerequisite[0] as usize, prerequisite[1] as usize);

            prerequisites_set.insert((a, b));
            is_prerequisite[a][b] = true;
            indegree[b] += 1;
        }

        for a in 0..num_courses {
            if indegree[a] == 0 {
                stack.push(a);
            }
        }

        while let Some(a) = stack.pop() {
            for b in 0..num_courses {
                if is_prerequisite[a][b] {
                    for c in 0..num_courses {
                        is_prerequisite[c][b] |= is_prerequisite[c][a];
                    }
                }

                if prerequisites_set.contains(&(a, b)) {
                    indegree[b] -= 1;
                    if indegree[b] == 0 {
                        stack.push(b);
                    }
                }
            }
        }

        queries
            .iter()
            .map(|query| is_prerequisite[query[0] as usize][query[1] as usize])
            .collect()
    }
}
