# 210. Course Schedule II
There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where <code>prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that you **must** take course <code>b<sub>i</sub></code> first if you want to take course <code>a<sub>i</sub></code>.

* For example, the pair `[0, 1]`, indicates that to take course `0` you have to first take course `1`.

Return *the ordering of courses you should take to finish all courses*. If there are many valid answers, return **any** of them. If it is impossible to finish all courses, return **an empty array**.

#### Example 1:
<pre>
<strong>Input:</strong> numCourses = 2, prerequisites = [[1,0]]
<strong>Output:</strong> [0,1]
<strong>Explanation:</strong> There are a total of 2 courses to take. To take course 1 you should have finished course 0. So the correct course order is [0,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
<strong>Output:</strong> [0,2,1,3]
<strong>Explanation:</strong> There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2. Both courses 1 and 2 should be taken after you finished course 0.
So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> numCourses = 1, prerequisites = []
<strong>Output:</strong> [0]
</pre>

#### Constraints:
* `1 <= numCourses <= 2000`
* `0 <= prerequisites.length <= numCourses * (numCourses - 1)`
* `prerequisites[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < numCourses</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* All the pairs <code>[a<sub>i</sub>, b<sub>i</sub>]</code> are distinct.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indegrees = vec![0; num_courses as usize];
        let mut follows = HashMap::new();
        let mut courses = vec![];
        let mut ret = vec![];

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
            ret.push(b as i32);

            for a in follows.remove(&b).unwrap_or(vec![]) {
                indegrees[a] -= 1;
                if indegrees[a] == 0 {
                    courses.push(a);
                }
            }
        }

        if ret.len() < num_courses as usize {
            ret.clear();
        }

        ret
    }
}
```
