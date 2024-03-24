# 1838. 最高频元素的频数
元素的 **频数** 是该元素在一个数组中出现的次数。

给你一个整数数组 `nums` 和一个整数 `k` 。在一步操作中，你可以选择 `nums` 的一个下标，并将该下标对应元素的值增加 `1` 。

执行最多 `k` 次操作后，返回数组中最高频元素的 **最大可能频数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,4], k = 5
<strong>输出:</strong> 3
<strong>解释:</strong> 对第一个元素执行 3 次递增操作，对第二个元素执 2 次递增操作，此时 nums = [4,4,4] 。
4 是数组中最高频元素，频数是 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,4,8,13], k = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 存在多种最优解决方案：
- 对第一个元素执行 3 次递增操作，此时 nums = [4,4,8,13] 。4 是数组中最高频元素，频数是 2 。
- 对第二个元素执行 4 次递增操作，此时 nums = [1,8,8,13] 。8 是数组中最高频元素，频数是 2 。
- 对第三个元素执行 5 次递增操作，此时 nums = [1,4,13,13] 。13 是数组中最高频元素，频数是 2 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,9,6], k = 2
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>
* <code>1 <= k <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut sum;
        let mut ret = 1;

        nums.sort_unstable();
        sum = nums[0] as i64;

        for j in 1..nums.len() {
            while (j - i) as i64 * nums[j] as i64 - sum > k as i64 {
                sum -= nums[i] as i64;
                i += 1;
            }

            sum += nums[j] as i64;
            ret = ret.max(j - i + 1);
        }

        ret as i32
    }
}
```
