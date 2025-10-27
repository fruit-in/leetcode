# 2127. 参加会议的最多员工数
一个公司准备组织一场会议，邀请名单上有 `n` 位员工。公司准备了一张 **圆形** 的桌子，可以坐下 **任意数目** 的员工。

员工编号为 `0` 到 `n - 1` 。每位员工都有一位 **喜欢** 的员工，每位员工 **当且仅当** 他被安排在喜欢员工的旁边，他才会参加会议。每位员工喜欢的员工 **不会** 是他自己。

给你一个下标从 **0** 开始的整数数组 `favorite` ，其中 `favorite[i]` 表示第 `i` 位员工喜欢的员工。请你返回参加会议的 **最多员工数目** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/14/ex1.png)
<pre>
<strong>输入:</strong> favorite = [2,2,1,2]
<strong>输出:</strong> 3
<strong>解释:</strong>
上图展示了公司邀请员工 0，1 和 2 参加会议以及他们在圆桌上的座位。
没办法邀请所有员工参与会议，因为员工 2 没办法同时坐在 0，1 和 3 员工的旁边。
注意，公司也可以邀请员工 1，2 和 3 参加会议。
所以最多参加会议的员工数目为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> favorite = [1,2,0]
<strong>输出:</strong> 3
<strong>解释:</strong>
每个员工都至少是另一个员工喜欢的员工。所以公司邀请他们所有人参加会议的前提是所有人都参加了会议。
座位安排同图 1 所示：
- 员工 0 坐在员工 2 和 1 之间。
- 员工 1 坐在员工 0 和 2 之间。
- 员工 2 坐在员工 1 和 0 之间。
参与会议的最多员工数目为 3 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/12/14/ex2.png)
<pre>
<strong>输入:</strong> favorite = [3,0,1,4,1]
<strong>输出:</strong> 4
<strong>解释:</strong>
上图展示了公司可以邀请员工 0，1，3 和 4 参加会议以及他们在圆桌上的座位。
员工 2 无法参加，因为他喜欢的员工 1 旁边的座位已经被占领了。
所以公司只能不邀请员工 2 。
参加会议的最多员工数目为 4 。
</pre>

#### 提示:
* `n == favorite.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `0 <= favorite[i] <= n - 1`
* `favorite[i] != i`

## 题解 (Rust)

### 1. 题解
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
