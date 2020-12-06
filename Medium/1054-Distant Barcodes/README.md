# 1054. Distant Barcodes
In a warehouse, there is a row of barcodes, where the <code>i<sup>th</sup></code> barcode is `barcodes[i]`.

Rearrange the barcodes so that no two adjacent barcodes are equal. You may return any answer, and it is guaranteed an answer exists.

#### Example 1:
<pre>
<strong>Input:</strong> barcodes = [1,1,1,2,2,2]
<strong>Output:</strong> [2,1,2,1,2,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> barcodes = [1,1,1,1,2,2,3,3]
<strong>Output:</strong> [1,3,1,3,1,2,1,2]
</pre>

#### Constraints:
* `1 <= barcodes.length <= 10000`
* `1 <= barcodes[i] <= 10000`

## Solutions (Rust)

### 1. Heap
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
