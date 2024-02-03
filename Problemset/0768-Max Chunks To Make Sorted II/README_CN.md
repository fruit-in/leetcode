# 768. 最多能完成排序的块 II
给你一个整数数组 `arr` 。

将 `arr` 分割成若干 **块** ，并将这些块分别进行排序。之后再连接起来，使得连接的结果和按升序排序后的原数组相同。

返回能将数组分成的最多块数？

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [5,4,3,2,1]
<strong>输出:</strong> 1
<strong>解释:</strong>
将数组分成2块或者更多块，都无法得到所需的结果。
例如，分成 [5, 4], [3, 2, 1] 的结果是 [4, 5, 1, 2, 3]，这不是有序的数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [2,1,3,4,4]
<strong>输出:</strong> 4
<strong>解释:</strong>
可以把它分成两块，例如 [2, 1], [3, 4, 4]。
然而，分成 [2, 1], [3], [4], [4] 可以得到最多的块数。
</pre>

#### 提示:
* `1 <= arr.length <= 2000`
* <code>0 <= arr[i] <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut heap = arr.iter().collect::<BinaryHeap<_>>();
        let mut pop_count = HashMap::new();
        let mut min_num = i32::MAX;
        let mut ret = 0;

        for i in (0..arr.len()).rev() {
            *pop_count.entry(arr[i]).or_insert(0) += 1;
            min_num = min_num.min(arr[i]);

            while *pop_count.get(*heap.peek().unwrap_or(&&-1)).unwrap_or(&0) > 0 {
                *pop_count.get_mut(heap.pop().unwrap()).unwrap() -= 1;
            }

            if **heap.peek().unwrap_or(&&-1) <= min_num {
                ret += 1;
            }
        }

        ret
    }
}
```
