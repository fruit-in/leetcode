# 1601. 最多可达成的换楼请求数目
我们有 `n` 栋楼，编号从 `0` 到 `n - 1` 。每栋楼有若干员工。由于现在是换楼的季节，部分员工想要换一栋楼居住。

给你一个数组 `requests` ，其中 <code>requests[i] = [from<sub>i</sub>, to<sub>i</sub>]</code> ，表示一个员工请求从编号为 <code>from<sub>i</sub></code> 的楼搬到编号为 <code>to<sub>i</sub></code> 的楼。

一开始 **所有楼都是满的**，所以从请求列表中选出的若干个请求是可行的需要满足 **每栋楼员工净变化为 0** 。意思是每栋楼 **离开** 的员工数目 **等于** 该楼 **搬入** 的员工数数目。比方说 `n = 3` 且两个员工要离开楼 `0` ，一个员工要离开楼 `1` ，一个员工要离开楼 `2` ，如果该请求列表可行，应该要有两个员工搬入楼 `0` ，一个员工搬入楼 `1` ，一个员工搬入楼 `2` 。

请你从原请求列表中选出若干个请求，使得它们是一个可行的请求列表，并返回所有可行列表中最大请求数目。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/10/move1.jpg)
<pre>
<strong>输入:</strong> n = 5, requests = [[0,1],[1,0],[0,1],[1,2],[2,0],[3,4]]
<strong>输出:</strong> 5
<strong>解释:</strong> 请求列表如下：
从楼 0 离开的员工为 x 和 y ，且他们都想要搬到楼 1 。
从楼 1 离开的员工为 a 和 b ，且他们分别想要搬到楼 2 和 0 。
从楼 2 离开的员工为 z ，且他想要搬到楼 0 。
从楼 3 离开的员工为 c ，且他想要搬到楼 4 。
没有员工从楼 4 离开。
我们可以让 x 和 b 交换他们的楼，以满足他们的请求。
我们可以让 y，a 和 z 三人在三栋楼间交换位置，满足他们的要求。
所以最多可以满足 5 个请求。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/09/10/move2.jpg)
<pre>
<strong>输入:</strong> n = 3, requests = [[0,0],[1,2],[2,1]]
<strong>输出:</strong> 3
<strong>解释:</strong> 请求列表如下：
从楼 0 离开的员工为 x ，且他想要回到原来的楼 0 。
从楼 1 离开的员工为 y ，且他想要搬到楼 2 。
从楼 2 离开的员工为 z ，且他想要搬到楼 1 。
我们可以满足所有的请求。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 4, requests = [[0,3],[3,1],[1,2],[2,0]]
<strong>输出:</strong> 4
</pre>

#### 提示:
* `1 <= n <= 20`
* `1 <= requests.length <= 16`
* `requests[i].length == 2`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub> < n</code>

## 题解 (Rust)

### 1. 题解
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
