# 1462. Course Schedule IV
There are a total of `numCourses` courses you have to take, labeled from `0` to `numCourses - 1`. You are given an array `prerequisites` where <code>prerequisites[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that you **must** take course <code>a<sub>i</sub></code> first if you want to take course <code>b<sub>i</sub></code>.

* For example, the pair `[0, 1]` indicates that you have to take course `0` before you can take course `1`.

Prerequisites can also be **indirect**. If course `a` is a prerequisite of course `b`, and course `b` is a prerequisite of course `c`, then course `a` is a prerequisite of course `c`.

You are also given an array `queries` where <code>queries[j] = [u<sub>j</sub>, v<sub>j</sub>]</code>. For the <code>j<sup>th</sup></code> query, you should answer whether course <code>u<sub>j</sub></code> is a prerequisite of course <code>v<sub>j</sub></code> or not.

Return *a boolean array* `answer`*, where* `answer[j]` *is the answer to the* <code>j<sup>th</sup></code> *query*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/05/01/courses4-1-graph.jpg)
<pre>
<strong>Input:</strong> numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
<strong>Output:</strong> [false,true]
<strong>Explanation:</strong> The pair [1, 0] indicates that you have to take course 1 before you can take course 0.
Course 0 is not a prerequisite of course 1, but the opposite is true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
<strong>Output:</strong> [false,false]
<strong>Explanation:</strong> There are no prerequisites, and each course is independent.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/05/01/courses4-3-graph.jpg)
<pre>
<strong>Input:</strong> numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
<strong>Output:</strong> [true,true]
</pre>

#### Constraints:
* `2 <= numCourses <= 100`
* `0 <= prerequisites.length <= (numCourses * (numCourses - 1) / 2)`
* `prerequisites[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= numCourses - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* All the pairs [ai, bi] are unique.
* The prerequisites graph has no cycles.
* <code>1 <= queries.length <= 10<sup>4</sup></code>
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> <= numCourses - 1</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>

## Solutions (Rust)

### 1. Solution
```Rust
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
```
