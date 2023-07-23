# 1414. 和为 K 的最少斐波那契数字数目
给你数字 `k` ，请你返回和为 `k` 的斐波那契数字的最少数目，其中，每个斐波那契数字都可以被使用多次。

斐波那契数字定义为：
* F<sub>1</sub> = 1
* F<sub>2</sub> = 1
* F<sub>n</sub> = F<sub>n-1</sub> + F<sub>n-2</sub> ， 其中 n > 2 。

数据保证对于给定的 `k` ，一定能找到可行解。

#### 示例 1:
<pre>
<strong>输入:</strong> k = 7
<strong>输出:</strong> 2
<strong>解释:</strong> 斐波那契数字为：1，1，2，3，5，8，13，……
对于 k = 7 ，我们可以得到 2 + 5 = 7 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> k = 10
<strong>输出:</strong> 2
<strong>解释:</strong> 对于 k = 10 ，我们可以得到 2 + 8 = 10 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> k = 19
<strong>输出:</strong> 3
<strong>解释:</strong> 对于 k = 19 ，我们可以得到 1 + 5 + 13 = 19 。
</pre>

#### 提示:
* `1 <= k <= 10^9`

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer} k
# @return {Integer}
def find_min_fibonacci_numbers(k)
  nums = [1, 1]
  ret = 0

  nums.push(nums[-2] + nums[-1]) while nums[-1] < k

  while k > 0
    nums.pop while nums[-1] > k
    k -= nums[-1]
    ret += 1
  end

  ret
end
```

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
        let mut nums = vec![1, 1];
        let mut i = 1;
        let mut ret = 0;

        while nums[i] < k {
            nums.push(nums[i - 1] + nums[i]);
            i += 1;
        }

        while k > 0 {
            while nums[i] > k {
                i -= 1;
            }
            k -= nums[i];
            ret += 1;
        }

        ret
    }
}
```
