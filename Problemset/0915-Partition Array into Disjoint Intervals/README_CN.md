# 915. 分割数组
给定一个数组 `A`，将其划分为两个不相交（没有公共元素）的连续子数组 `left` 和 `right`， 使得：
* `left` 中的每个元素都小于或等于 `right` 中的每个元素。
* `left` 和 `right` 都是非空的。
* `left` 要尽可能小。

在完成这样的分组后返回 `left` 的**长度**。可以保证存在这样的划分方法。

#### 示例 1:
<pre>
<strong>输入:</strong> [5,0,3,8,6]
<strong>输出:</strong> 3
<strong>解释:</strong> left = [5,0,3], right = [8,6]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,1,1,0,6,12]
<strong>输出:</strong> 4
<strong>解释:</strong> left = [1,1,1,0], right = [6,12]
</pre>

#### 提示:
1. `2 <= A.length <= 30000`
2. `0 <= A[i] <= 10^6`
3. 可以保证至少有一种方法能够按题目所描述的那样对 `A` 进行划分。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} a
# @return {Integer}
def partition_disjoint(a)
  max_left = a[0]
  max = a[0]
  length = 1

  (1...a.length).each do |i|
    if a[i] < max_left
      length = i + 1
      max_left = max
    elsif a[i] > max
      max = a[i]
    end
  end

  length
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn partition_disjoint(a: Vec<i32>) -> i32 {
        let mut max_left = a[0];
        let mut max = a[0];
        let mut length = 1;

        for i in 1..a.len() {
            if a[i] < max_left {
                length = i + 1;
                max_left = max;
            } else if a[i] > max {
                max = a[i];
            }
        }

        length as i32
    }
}
```
