# 2127. Maximum Employees to Be Invited to a Meeting
A company is organizing a meeting and has a list of `n` employees, waiting to be invited. They have arranged for a large **circular** table, capable of seating **any number** of employees.

The employees are numbered from `0` to `n - 1`. Each employee has a **favorite** person and they will attend the meeting **only if** they can sit next to their favorite person at the table. The favorite person of an employee is **not** themself.

Given a **0-indexed** integer array `favorite`, where `favorite[i]` denotes the favorite person of the <code>i<sup>th</sup></code> employee, return *the **maximum number of employees** that can be invited to the meeting*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/14/ex1.png)
<pre>
<strong>Input:</strong> favorite = [2,2,1,2]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
The above figure shows how the company can invite employees 0, 1, and 2, and seat them at the round table.
All employees cannot be invited because employee 2 cannot sit beside employees 0, 1, and 3, simultaneously.
Note that the company can also invite employees 1, 2, and 3, and give them their desired seats.
The maximum number of employees that can be invited to the meeting is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> favorite = [1,2,0]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Each employee is the favorite person of at least one other employee, and the only way the company can invite them is if they invite every employee.
The seating arrangement will be the same as that in the figure given in example 1:
- Employee 0 will sit between employees 2 and 1.
- Employee 1 will sit between employees 0 and 2.
- Employee 2 will sit between employees 1 and 0.
The maximum number of employees that can be invited to the meeting is 3.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/12/14/ex2.png)
<pre>
<strong>Input:</strong> favorite = [3,0,1,4,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
The above figure shows how the company will invite employees 0, 1, 3, and 4, and seat them at the round table.
Employee 2 cannot be invited because the two spots next to their favorite employee 1 are taken.
So the company leaves them out of the meeting.
The maximum number of employees that can be invited to the meeting is 4.
</pre>

#### Constraints:
* `n == favorite.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `0 <= favorite[i] <= n - 1`
* `favorite[i] != i`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let favorite = favorite.iter().map(|&fav| fav as usize).collect::<Vec<_>>();
        let n = favorite.len();
        let mut indegree = vec![0; n];
        let mut stack = vec![];
        let mut incycle = vec![true; n];
        let mut chainlength = vec![1; n];
        let mut couplesum = 0;
        let mut cyclemax = 0;

        for i in 0..n {
            indegree[favorite[i]] += 1;
        }

        for i in 0..n {
            if indegree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(i) = stack.pop() {
            indegree[favorite[i]] -= 1;
            incycle[i] = false;
            chainlength[favorite[i]] = chainlength[favorite[i]].max(chainlength[i] + 1);
            if indegree[favorite[i]] == 0 {
                stack.push(favorite[i]);
            }
        }

        for i in 0..n {
            if incycle[i] {
                if favorite[favorite[i]] == i {
                    couplesum += chainlength[i];
                } else {
                    let mut j = favorite[i];
                    let mut length = 1;

                    while j != i {
                        incycle[j] = false;
                        j = favorite[j];
                        length += 1;
                    }

                    cyclemax = cyclemax.max(length);
                }
            }
        }

        couplesum.max(cyclemax)
    }
}
```
