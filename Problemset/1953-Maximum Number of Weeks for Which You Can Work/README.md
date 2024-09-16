# 1953. Maximum Number of Weeks for Which You Can Work
There are `n` projects numbered from `0` to `n - 1`. You are given an integer array `milestones` where each `milestones[i]` denotes the number of milestones the <code>i<sup>th</sup></code> project has.

You can work on the projects following these two rules:

* Every week, you will finish **exactly one** milestone of **one** project. You **must** work every week.
* You **cannot** work on two milestones from the same project for two **consecutive** weeks.

Once all the milestones of all the projects are finished, or if the only milestones that you can work on will cause you to violate the above rules, you will **stop working**. Note that you may not be able to finish every project's milestones due to these constraints.

Return *the **maximum** number of weeks you would be able to work on the projects without violating the rules mentioned above*.

#### Example 1:
<pre>
<strong>Input:</strong> milestones = [1,2,3]
<strong>Output:</strong> 6
<strong>Explanation:</strong> One possible scenario is:
- During the 1st week, you will work on a milestone of project 0.
- During the 2nd week, you will work on a milestone of project 2.
- During the 3rd week, you will work on a milestone of project 1.
- During the 4th week, you will work on a milestone of project 2.
- During the 5th week, you will work on a milestone of project 1.
- During the 6th week, you will work on a milestone of project 2.
The total number of weeks is 6.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> milestones = [5,2,1]
<strong>Output:</strong> 7
<strong>Explanation:</strong> One possible scenario is:
- During the 1st week, you will work on a milestone of project 0.
- During the 2nd week, you will work on a milestone of project 1.
- During the 3rd week, you will work on a milestone of project 0.
- During the 4th week, you will work on a milestone of project 1.
- During the 5th week, you will work on a milestone of project 0.
- During the 6th week, you will work on a milestone of project 2.
- During the 7th week, you will work on a milestone of project 0.
The total number of weeks is 7.
Note that you cannot work on the last milestone of project 0 on 8th week because it would violate the rules.
Thus, one milestone in project 0 will remain unfinished.
</pre>

#### Constraints:
* `n == milestones.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= milestones[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let max = *milestones.iter().max().unwrap() as i64;
        let sum = milestones.iter().map(|&x| x as i64).sum::<i64>();

        if max * 2 - 1 > sum {
            (sum - max) * 2 + 1
        } else {
            sum
        }
    }
}
```
