# 1524. 和为奇数的子数组数目
给你一个整数数组 `arr` 。请你返回和为 **奇数** 的子数组数目。

由于答案可能会很大，请你将结果对 `10^9 + 7` 取余后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,3,5]
<strong>输出:</strong> 4
<strong>解释:</strong> 所有的子数组为 [[1],[1,3],[1,3,5],[3],[3,5],[5]] 。
所有子数组的和为 [1,4,9,3,8,5].
奇数和包括 [1,9,3,5] ，所以答案为 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [2,4,6]
<strong>输出:</strong> 0
<strong>解释:</strong> 所有子数组为 [[2],[2,4],[2,4,6],[4],[4,6],[6]] 。
所有子数组和为 [2,6,12,4,10,6] 。
所有子数组和都是偶数，所以答案为 0 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [1,2,3,4,5,6,7]
<strong>输出:</strong> 16
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> arr = [100,100,99,99]
<strong>输出:</strong> 4
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> arr = [7]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= arr.length <= 10^5`
* `1 <= arr[i] <= 100`

## 题解 (Ruby)

### 1. 前缀和
```Ruby
# @param {Integer[]} arr
# @return {Integer}
def num_of_subarrays(arr)
  sum = 0
  odd = 0
  even = 1
  ret = 0

  arr.each do |x|
    sum += x
    if sum.odd?
      odd += 1
      ret += even
    else
      even += 1
      ret += odd
    end
  end

  ret % 1_000_000_007
end
```

## 题解 (Rust)

### 1. 前缀和
```Rust
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut odd = 0;
        let mut even = 1;
        let mut ret = 0;

        for x in arr {
            sum += x;
            if sum % 2 == 1 {
                odd += 1;
                ret = (ret + even) % 1_000_000_007;
            } else {
                even += 1;
                ret = (ret + odd) % 1_000_000_007;
            }
        }

        ret
    }
}
```
