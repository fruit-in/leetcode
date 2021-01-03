# 870. 优势洗牌
给定两个大小相等的数组 `A` 和 `B`，`A` 相对于 `B` 的*优势*可以用满足 `A[i] > B[i]` 的索引 `i` 的数目来描述。

返回 `A` 的**任意**排列，使其相对于 `B` 的优势最大化。

#### 示例 1:
<pre>
<strong>输入:</strong> A = [2,7,11,15], B = [1,10,4,11]
<strong>输出:</strong> [2,11,7,15]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> A = [12,24,8,32], B = [13,25,32,11]
<strong>输出:</strong> [24,32,8,12]
</pre>

#### 提示:
1. `1 <= A.length = B.length <= 10000`
2. `0 <= A[i] <= 10^9`
3. `0 <= B[i] <= 10^9`

## 题解 (Ruby)

### 1. 贪心
```Ruby
# @param {Integer[]} a
# @param {Integer[]} b
# @return {Integer[]}
def advantage_count(a, b)
  i = 0
  j = b.length - 1
  indices = (0...b.length).to_a
  ret = Array.new(b.length)

  a.sort!
  indices.sort_by! { |k| -b[k] }

  indices.each do |k|
    if b[k] < a[j]
      ret[k] = a[j]
      j -= 1
    else
      ret[k] = a[i]
      i += 1
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn advantage_count(mut a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = b.len() - 1;
        let mut indices = (0..b.len()).collect::<Vec<_>>();
        let mut ret = vec![0; b.len()];

        a.sort_unstable();
        indices.sort_unstable_by_key(|&k| -b[k]);

        for k in indices {
            if b[k] < a[j] {
                ret[k] = a[j];
                j -= 1;
            } else {
                ret[k] = a[i];
                i += 1;
            }
        }

        ret
    }
}
```
