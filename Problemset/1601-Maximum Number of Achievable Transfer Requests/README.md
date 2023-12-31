# 1601. Maximum Number of Achievable Transfer Requests
We have `n` buildings numbered from `0` to `n - 1`. Each building has a number of employees. It's transfer season, and some employees want to change the building they reside in.

You are given an array `requests` where <code>requests[i] = [from<sub>i</sub>, to<sub>i</sub>]</code> represents an employee's request to transfer from building <code>from<sub>i</sub></code> to building <code>to<sub>i</sub></code>.

**All buildings are full**, so a list of requests is achievable only if for each building, the **net change in employee transfers is zero**. This means the number of employees **leaving** is **equal** to the number of employees **moving in**. For example if `n = 3` and two employees are leaving building `0`, one is leaving building `1`, and one is leaving building `2`, there should be two employees moving to building `0`, one employee moving to building `1`, and one employee moving to building `2`.

Return *the maximum number of achievable requests*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/10/move1.jpg)
<pre>
<strong>Input:</strong> n = 5, requests = [[0,1],[1,0],[0,1],[1,2],[2,0],[3,4]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> Let's see the requests:
From building 0 we have employees x and y and both want to move to building 1.
From building 1 we have employees a and b and they want to move to buildings 2 and 0 respectively.
From building 2 we have employee z and they want to move to building 0.
From building 3 we have employee c and they want to move to building 4.
From building 4 we don't have any requests.
We can achieve the requests of users x and b by swapping their places.
We can achieve the requests of users y, a and z by swapping the places in the 3 buildings.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/09/10/move2.jpg)
<pre>
<strong>Input:</strong> n = 3, requests = [[0,0],[1,2],[2,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> Let's see the requests:
From building 0 we have employee x and they want to stay in the same building 0.
From building 1 we have employee y and they want to move to building 2.
From building 2 we have employee z and they want to move to building 1.
We can achieve all the requests.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4, requests = [[0,3],[3,1],[1,2],[2,0]]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `1 <= n <= 20`
* `1 <= requests.length <= 16`
* `requests[i].length == 2`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub> < n</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let limit = (1 << requests.len()) - 1;

        for m in (1..=requests.len()).rev() {
            let mut x: i32 = (1 << m) - 1;

            while x <= limit {
                let mut change = vec![0; n as usize];

                for i in 0..requests.len() {
                    if x & (1 << i) != 0 {
                        change[requests[i][0] as usize] -= 1;
                        change[requests[i][1] as usize] += 1;
                    }
                }

                if change.iter().all(|&c| c == 0) {
                    return m as i32;
                }

                let zeros = x.trailing_zeros();
                let ones = (x >> zeros).trailing_ones();
                x = (((x >> (zeros + ones)) + 1) << (zeros + ones)) + (1 << (ones - 1)) - 1;
            }
        }

        0
    }
}
```
