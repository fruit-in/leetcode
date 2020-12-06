# 1054. 距离相等的条形码
在一个仓库里，有一排条形码，其中第 `i` 个条形码为 `barcodes[i]`。

请你重新排列这些条形码，使其中两个相邻的条形码 **不能** 相等。 你可以返回任何满足该要求的答案，此题保证存在答案。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,1,1,2,2,2]
<strong>输出:</strong> [2,1,2,1,2,1]
</pre>


#### 示例 2:
<pre>
<strong>输入:</strong> [1,1,1,1,2,2,3,3]
<strong>输出:</strong> [1,3,1,3,1,2,1,2]
</pre>


#### 提示:
1. `1 <= barcodes.length <= 10000`
2. `1 <= barcodes[i] <= 10000`

## 题解 (Rust)

### 1. 堆
```Rust
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        let mut count = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut ret = vec![];

        for bar in barcodes {
            *count.entry(bar).or_insert(0) += 1;
        }
        for (k, v) in count.into_iter() {
            heap.push((v, k));
        }

        while let Some(mut max0) = heap.pop() {
            ret.push(max0.1);
            max0.0 -= 1;
            if let Some(mut max1) = heap.pop() {
                ret.push(max1.1);
                max1.0 -= 1;
                if max1.0 > 0 {
                    heap.push(max1);
                }
            }
            if max0.0 > 0 {
                heap.push(max0);
            }
        }

        ret
    }
}
```
