# 565. 数组嵌套
索引从`0`开始长度为`N`的数组`A`，包含`0`到`N - 1`的所有整数。找到最大的集合`S`并返回其大小，其中 `S[i] = {A[i], A[A[i]], A[A[A[i]]], ... }`且遵守以下的规则。

假设选择索引为`i`的元素`A[i]`为`S`的第一个元素，`S`的下一个元素应该是`A[A[i]]`，之后是`A[A[A[i]]]...` 以此类推，不断添加直到`S`出现重复的元素。

#### 示例 1:
<pre>
<strong>输入:</strong> A = [5,4,0,3,1,6,2]
<strong>输出:</strong> 4
<strong>解释:</strong>
A[0] = 5, A[1] = 4, A[2] = 0, A[3] = 3, A[4] = 1, A[5] = 6, A[6] = 2.

其中一种最长的 S[K]:
S[0] = {A[0], A[5], A[6], A[2]} = {5, 6, 2, 0}
</pre>

#### 提示:
1. `N`是`[1, 20,000]`之间的整数。
2. `A`中不含有重复的元素。
3. `A`中的元素大小在`[0, N-1]`之间。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def array_nesting(nums)
  ret = 1

  (0...nums.size).each do |i|
    next if nums[i] < 0

    length = 0
    j = i
    while nums[j] >= 0
      nums[j] = -nums[j] - 1
      length += 1
      j = -nums[j] - 1
    end

    ret = [ret, length].max
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let mut ret = 1;

        for i in 0..nums.len() {
            if nums[i] < 0 {
                continue;
            }

            let mut length = 0;
            let mut j = i;
            while nums[j] >= 0 {
                nums[j] = -nums[j] - 1;
                length += 1;
                j = (-nums[j] - 1) as usize;
            }

            ret = ret.max(length);
        }

        ret
    }
}
```
