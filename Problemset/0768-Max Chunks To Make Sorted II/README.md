# 768. Max Chunks To Make Sorted II
You are given an integer array `arr`.

We split `arr` into some number of **chunks** (i.e., partitions), and individually sort each chunk. After concatenating them, the result should equal the sorted array.

Return *the largest number of chunks we can make to sort the array*.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [5,4,3,2,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Splitting into two or more chunks will not return the required result.
For example, splitting into [5, 4], [3, 2, 1] will result in [4, 5, 1, 2, 3], which isn't sorted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [2,1,3,4,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
We can split into two chunks, such as [2, 1], [3, 4, 4].
However, splitting into [2, 1], [3], [4], [4] is the highest number of chunks possible.
</pre>

#### Constraints:
* `1 <= arr.length <= 2000`
* <code>0 <= arr[i] <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Solution
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
