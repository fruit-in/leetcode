# 1671. 得到山形数组的最少删除次数
我们定义 `arr` 是 **山形数组** 当且仅当它满足：
* `arr.length >= 3`
* 存在某个下标 `i` （**从 0 开始**） 满足 `0 < i < arr.length - 1` 且：
    * `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
    * `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`

给你整数数组 `nums` ，请你返回将 `nums` 变成 **山形状数组** 的 **最少** 删除次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 数组本身就是山形数组，所以我们不需要删除任何元素。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,1,1,5,6,2,3,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 一种方法是将下标为 0，1 和 5 的元素删除，剩余元素为 [1,5,6,3,1] ，是山形数组。
</pre>

#### 提示:
* `3 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* 题目保证 `nums` 删除一些元素后一定能得到山形数组。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut lis_last = vec![0];
        let mut lis_left = vec![0; nums.len()];
        let mut lis_right = vec![0; nums.len()];

        for i in 0..nums.len() {
            let j = match lis_last.binary_search(&nums[i]) {
                Ok(j) => j,
                Err(j) if j == lis_last.len() => {
                    lis_last.push(0);
                    j
                }
                Err(j) => j,
            };

            lis_last[j] = nums[i];
            lis_left[i] = j;
        }

        lis_last = vec![0];
        for i in (0..nums.len()).rev() {
            let j = match lis_last.binary_search(&nums[i]) {
                Ok(j) => j,
                Err(j) if j == lis_last.len() => {
                    lis_last.push(0);
                    j
                }
                Err(j) => j,
            };

            lis_last[j] = nums[i];
            lis_right[i] = j;
        }

        (0..nums.len())
            .filter(|&i| lis_left[i] > 1 && lis_right[i] > 1)
            .map(|i| nums.len() + 1 - lis_left[i] - lis_right[i])
            .min()
            .unwrap() as i32
    }
}
```
