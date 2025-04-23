# 2532. 过桥的时间
共有 `k` 位工人计划将 `n` 个箱子从右侧的（旧）仓库移动到左侧的（新）仓库。给你两个整数 `n` 和 `k`，以及一个二维整数数组 `time` ，数组的大小为 `k x 4` ，其中 <code>time[i] = [right<sub>i</sub>, pick<sub>i</sub>, left<sub>i</sub>, put<sub>i</sub>]</code> 。

一条河将两座仓库分隔，只能通过一座桥通行。旧仓库位于河的右岸，新仓库在河的左岸。开始时，所有 `k` 位工人都在桥的左侧等待。为了移动这些箱子，第 `i` 位工人（下标从 **0** 开始）可以：
* 从左岸（新仓库）跨过桥到右岸（旧仓库），用时 <code>right<sub>i</sub></code> 分钟。
* 从旧仓库选择一个箱子，并返回到桥边，用时 picki 分钟。不同工人可以同时搬起所选的箱子。
* 从右岸（旧仓库）跨过桥到左岸（新仓库），用时 <code>left<sub>i</sub></code> 分钟。
* 将箱子放入新仓库，并返回到桥边，用时 puti 分钟。不同工人可以同时放下所选的箱子。

如果满足下面任一条件，则认为工人 `i` 的 **效率低于** 工人 `j` ：
* <code>left<sub>i</sub> + right<sub>i</sub> > left<sub>j</sub> + right<sub>j</sub></code>
* <code>left<sub>i</sub> + right<sub>i</sub> == left<sub>j</sub> + right<sub>j</sub></code> 且 `i > j`

工人通过桥时需要遵循以下规则：
* 同时只能有一名工人过桥。
* 当桥梁未被使用时，优先让右侧 **效率最低** 的工人（已经拿起盒子的工人）过桥。如果不是，优先让左侧 **效率最低** 的工人通过。
* 如果左侧已经派出足够的工人来拾取所有剩余的箱子，则 **不会** 再从左侧派出工人。

请你返回最后一个箱子 **到达桥左侧** 的时间。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1, k = 3, time = [[1,1,2,1],[1,1,3,1],[1,1,4,1]]
<strong>输出:</strong> 6
<strong>解释:</strong>
从 0 到 1 分钟：工人 2 通过桥到达右侧。
从 1 到 2 分钟：工人 2 从右侧仓库拿起箱子。
从 2 到 6 分钟：工人 2 通过桥到达左侧。
从 6 到 7 分钟：工人 2 向左侧仓库放下箱子。
整个过程在 7 分钟后结束。我们返回 6 因为该问题要求的是最后一名工人到达桥梁左侧的时间。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, k = 2, time = [[1,5,1,8],[10,10,10,10]]
<strong>输出:</strong> 37
<strong>解释:</strong>
<img src="https://assets.leetcode.com/uploads/2024/11/21/378539249-c6ce3c73-40e7-4670-a8b5-7ddb9abede11.png">
最后一个盒子在37秒时到达左侧。请注意，我们并 没有 放下最后一个箱子，因为那样会花费更多时间，而且它们已经和工人们一起在左边。
</pre>

#### 提示:
* <code>1 <= n, k <= 10<sup>4</sup></code>
* `time.length == k`
* `time[i].length == 4`
* <code>1 <= left<sub>i</sub>, pick<sub>i</sub>, right<sub>i</sub>, put<sub>i</sub> <= 1000</code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut right_workers = BinaryHeap::new();
        let mut pick_workers = BinaryHeap::new();
        let mut left_workers = BinaryHeap::new();
        let mut put_workers = BinaryHeap::new();
        let mut pick = 0;
        let mut put = 0;
        let mut ret = 0;

        for i in 0..time.len() {
            left_workers.push((time[i][2] + time[i][0], i));
        }

        while put < n {
            while pick_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0 <= ret {
                let i = pick_workers.pop().unwrap().1;
                right_workers.push((time[i][2] + time[i][0], i));
            }
            while put < n && put_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0 <= ret {
                let i = put_workers.pop().unwrap().1;
                left_workers.push((time[i][2] + time[i][0], i));
            }

            if let Some((_, i)) = right_workers.pop() {
                ret += time[i][2];
                put_workers.push((Reverse(ret + time[i][3]), i));
                put += 1;
            } else if pick == n || left_workers.is_empty() {
                ret = pick_workers
                    .peek()
                    .unwrap_or(&(Reverse(i32::MAX), 0))
                    .0
                     .0
                    .min(put_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0);
            } else if let Some((_, i)) = left_workers.pop() {
                ret += time[i][0];
                pick_workers.push((Reverse(ret + time[i][1]), i));
                pick += 1;
            }
        }

        ret
    }
}
```
