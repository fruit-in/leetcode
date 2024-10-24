# 2202. K 次操作后最大化顶端元素
给你一个下标从 **0** 开始的整数数组 `nums` ，它表示一个 **堆** ，其中 `nums[0]` 是堆顶的元素。

每一次操作中，你可以执行以下操作 **之一** ：

* 如果堆非空，那么 **删除** 堆顶端的元素。
* 如果存在 1 个或者多个被删除的元素，你可以从它们中选择任何一个，**添加** 回堆顶，这个元素成为新的堆顶元素。

同时给你一个整数 `k` ，它表示你总共需要执行操作的次数。

请你返回 **恰好** 执行 `k` 次操作以后，堆顶元素的 **最大值** 。如果执行完 `k` 次操作以后，堆一定为空，请你返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,2,2,4,0,6], k = 4
<strong>输出:</strong> 5
<strong>解释:</strong>
4 次操作后，堆顶元素为 5 的方法之一为：
- 第 1 次操作：删除堆顶元素 5 ，堆变为 [2,2,4,0,6] 。
- 第 2 次操作：删除堆顶元素 2 ，堆变为 [2,4,0,6] 。
- 第 3 次操作：删除堆顶元素 2 ，堆变为 [4,0,6] 。
- 第 4 次操作：将 5 添加回堆顶，堆变为 [5,4,0,6] 。
注意，这不是最后堆顶元素为 5 的唯一方式。但可以证明，4 次操作以后 5 是能得到的最大堆顶元素。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2], k = 1
<strong>输出:</strong> -1
<strong>解释:</strong>
第 1 次操作中，我们唯一的选择是将堆顶元素弹出堆。
由于 1 次操作后无法得到一个非空的堆，所以我们返回 -1 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i], k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_top(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 && (k - nums.len() as i32) % 2 == 0 {
            -1
        } else if k == 0 {
            nums[0]
        } else if k < nums.len() as i32 {
            nums[k as usize].max(*nums.iter().take(k as usize - 1).max().unwrap_or(&0))
        } else {
            *nums.iter().take(k as usize - 1).max().unwrap()
        }
    }
}
```
