# 769. 最多能完成排序的块
数组`arr`是`[0, 1, ..., arr.length - 1]`的一种排列，我们将这个数组分割成几个“块”，并将这些块分别进行排序。之后再连接起来，使得连接的结果和按升序排序后的原数组相同。

我们最多能将数组分成多少块？

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [4,3,2,1,0]
<strong>输出:</strong> 1
<strong>解释:</strong>
将数组分成2块或者更多块，都无法得到所需的结果。
例如，分成 [4, 3], [2, 1, 0] 的结果是 [3, 4, 0, 1, 2]，这不是有序的数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,0,2,3,4]
<strong>输出:</strong> 4
<strong>解释:</strong>
我们可以把它分成两块，例如 [1, 0], [2, 3, 4]。
然而，分成 [1, 0], [2], [3], [4] 可以得到最多的块数。
</pre>

#### 注意:
* `arr` 的长度在 `[1, 10]` 之间。
* `arr[i]`是 `[0, 1, ..., arr.length - 1]`的一种排列。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} arr
# @return {Integer}
def max_chunks_to_sorted(arr)
  x = 0
  ret = 0

  (0...arr.size).each do |i|
    x ^= 1 << arr[i]
    ret += 1 if x == (2 << i) - 1
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut ret = 0;

        for i in 0..arr.len() {
            x ^= 1 << arr[i];
            if x == (2 << i) - 1 {
                ret += 1;
            }
        }

        ret
    }
}
```
