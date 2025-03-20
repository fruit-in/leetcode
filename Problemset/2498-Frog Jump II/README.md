# 2498. Frog Jump II
You are given a **0-indexed** integer array `stones` sorted in **strictly increasing order** representing the positions of stones in a river.

A frog, initially on the first stone, wants to travel to the last stone and then return to the first stone. However, it can jump to any stone **at most once**.

The **length** of a jump is the absolute difference between the position of the stone the frog is currently on and the position of the stone to which the frog jumps.

* More formally, if the frog is at `stones[i]` and is jumping to `stones[j]`, the length of the jump is `|stones[i] - stones[j]|`.

The **cost** of a path is the **maximum length of a jump** among all jumps in the path.

Return *the **minimum** cost of a path for the frog*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/11/14/example-1.png)
<pre>
<strong>Input:</strong> stones = [0,2,5,6,7]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The above figure represents one of the optimal paths the frog can take.
The cost of this path is 5, which is the maximum length of a jump.
Since it is not possible to achieve a cost of less than 5, we return it.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/11/14/example-2.png)
<pre>
<strong>Input:</strong> stones = [0,3,9]
<strong>Output:</strong> 9
<strong>Explanation:</strong>
The frog can jump directly to the last stone and come back to the first stone.
In this case, the length of each jump will be 9. The cost for the path will be max(9, 9) = 9.
It can be shown that this is the minimum achievable cost.
</pre>

#### Constraints:
* <code>2 <= stones.length <= 10<sup>5</sup></code>
* <code>0 <= stones[i] <= 10<sup>9</sup></code>
* `stones[0] == 0`
* `stones` is sorted in a strictly increasing order.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_jump(stones: Vec<i32>) -> i32 {
        let mut lo = 1;
        let mut hi = *stones.last().unwrap();

        while lo < hi {
            let mid = (lo + hi) / 2;
            let mut i = 0;
            let mut j = 1;
            let mut flag = true;
            let mut used = vec![false; stones.len()];

            while i < stones.len() - 1 {
                if stones[j] - stones[j - 1] > mid {
                    flag = false;
                    break;
                } else if j == stones.len() - 1 || stones[j + 1] - stones[i] > mid {
                    i = j;
                    used[i] = true;
                }

                j += 1;
            }

            if flag {
                used[i] = false;
                i = 0;
                j = 1;

                while i < stones.len() - 1 {
                    if stones[j] - stones[i] > mid {
                        flag = false;
                        break;
                    } else if !used[j] && stones[j] - stones[i] <= mid {
                        i = j;
                    }

                    j += 1;
                }
            }

            if flag {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        hi
    }
}
```
