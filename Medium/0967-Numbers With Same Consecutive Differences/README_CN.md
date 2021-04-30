# 967. 连续差相同的数字
返回所有长度为 `n` 且满足其每两个连续位上的数字之间的差的绝对值为 `k` 的 **非负整数** 。

请注意，**除了** 数字 `0` 本身之外，答案中的每个数字都 不能 有前导零。例如，`01` 有一个前导零，所以是无效的；但 `0` 是有效的。

你可以按 **任何顺序** 返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, k = 7
<strong>输出:</strong> [181,292,707,818,929]
<strong>解释:</strong> 注意，070 不是一个有效的数字，因为它有前导零。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, k = 1
<strong>输出:</strong> [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 2, k = 0
<strong>输出:</strong> [11,22,33,44,55,66,77,88,99]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 2, k = 2
<strong>输出:</strong> [13,20,24,31,35,42,46,53,57,64,68,75,79,86,97]
</pre>

#### 提示:
* `2 <= n <= 9`
* `0 <= k <= 9`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} n
# @param {Integer} k
# @return {Integer[]}
def nums_same_consec_diff(n, k)
  nums = (1..9).to_a

  (2..n).each do |_|
    nums_ = []

    nums.each do |x|
      y = x % 10
      nums_.push(x * 10 + y + k) if y + k < 10
      nums_.push(x * 10 + y - k) if y - k >= 0 && k != 0
    end

    nums = nums_
  end

  nums
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut nums = (1..10).collect();

        for _ in 1..n {
            let mut nums_ = vec![];

            for x in nums {
                let y = x % 10;
                if y + k < 10 {
                    nums_.push(x * 10 + y + k);
                }
                if y - k >= 0 && k != 0 {
                    nums_.push(x * 10 + y - k);
                }
            }

            nums = nums_;
        }

        nums
    }
}
```
