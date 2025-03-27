# 2050. Parallel Courses III
You are given an integer `n`, which indicates that there are `n` courses labeled from `1` to `n`. You are also given a 2D integer array `relations` where <code>relations[j] = [prevCourse<sub>j</sub>, nextCourse<sub>j</sub>]</code> denotes that course <code>prevCourse<sub>j</sub></code> has to be completed before course <code>nextCourse<sub>j</sub></code> (prerequisite relationship). Furthermore, you are given a 0-indexed integer array `time` where `time[i]` denotes how many months it takes to complete the <code>(i+1)<sup>th</sup></code> course.

You must find the **minimum** number of months needed to complete all the courses following these rules:
* You may start taking a course at **any time** if the prerequisites are met.
* **Any number of courses** can be taken at the **same time**.

Return *the **minimum** number of months needed to complete all the courses*.

**Note:** The test cases are generated such that it is possible to complete every course (i.e., the graph is a directed acyclic graph).

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/10/07/ex1.png)
<pre>
<strong>Input:</strong> n = 3, relations = [[1,3],[2,3]], time = [3,2,5]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The figure above represents the given graph and the time required to complete each course.
We start course 1 and course 2 simultaneously at month 0.
Course 1 takes 3 months and course 2 takes 2 months to complete respectively.
Thus, the earliest time we can start course 3 is at month 3, and the total time required is 3 + 5 = 8 months.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/10/07/ex2.png)
<pre>
<strong>Input:</strong> n = 5, relations = [[1,5],[2,5],[3,5],[3,4],[4,5]], time = [1,2,3,4,5]
<strong>Output:</strong> 12
<strong>Explanation:</strong> The figure above represents the given graph and the time required to complete each course.
You can start courses 1, 2, and 3 at month 0.
You can complete them after 1, 2, and 3 months respectively.
Course 4 can be taken only after course 3 is completed, i.e., after 3 months. It is completed after 3 + 4 = 7 months.
Course 5 can be taken only after courses 1, 2, 3, and 4 have been completed, i.e., after max(1,2,3,7) = 7 months.
Thus, the minimum time needed to complete all the courses is 7 + 5 = 12 months.
</pre>

#### Constraints:
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>0 <= relations.length <= min(n * (n - 1) / 2, 5 * 10<sup>4</sup>)</code>
* `relations[j].length == 2`
* <code>1 <= prevCourse<sub>j</sub>, nextCourse<sub>j</sub> <= n</code>
* <code>prevCourse<sub>j</sub> != nextCourse<sub>j</sub></code>
* All the pairs <code>[prevCourse<sub>j</sub>, nextCourse<sub>j</sub>]</code> are **unique**.
* `time.length == n`
* <code>1 <= time[i] <= 10<sup>4</sup></code>
* The given graph is a directed acyclic graph.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let mut next_courses = vec![vec![]; n as usize];
        let mut indgree = vec![0; n as usize];
        let mut start = vec![0; n as usize];
        let mut stack = vec![];
        let mut ret = 0;

        for relation in &relations {
            let (prev, next) = (relation[0] as usize - 1, relation[1] as usize - 1);

            next_courses[prev].push(next);
            indgree[next] += 1;
        }

        for i in 0..n as usize {
            if indgree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(prev) = stack.pop() {
            let end = start[prev] + time[prev];

            for &next in &next_courses[prev] {
                indgree[next] -= 1;
                start[next] = start[next].max(end);
                if indgree[next] == 0 {
                    stack.push(next);
                }
            }

            ret = ret.max(end);
        }

        ret
    }
}
```
