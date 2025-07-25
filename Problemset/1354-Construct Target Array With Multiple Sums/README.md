# 1354. Construct Target Array With Multiple Sums
You are given an array `target` of n integers. From a starting array `arr` consisting of `n` 1's, you may perform the following procedure :
* let `x` be the sum of all elements currently in your array.
* choose index `i`, such that `0 <= i < n` and set the value of `arr` at index `i` to `x`.
* You may repeat this procedure as many times as needed.

Return `true` *if it is possible to construct the* `target` *array from* `arr`, *otherwise, return* `false`.

#### Example 1:
<pre>
<strong>Input:</strong> target = [9,3,5]
<strong>Output:</strong> true
<strong>Explanation:</strong> Start with arr = [1, 1, 1]
[1, 1, 1], sum = 3 choose index 1
[1, 3, 1], sum = 5 choose index 2
[1, 3, 5], sum = 9 choose index 0
[9, 3, 5] Done
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = [1,1,1,2]
<strong>Output:</strong> false
<strong>Explanation:</strong> Impossible to create target array from [1,1,1,1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = [8,5]
<strong>Output:</strong> true
</pre>

#### Constraints:
* `n == target.length`
* <code>1 <= n <= 5 * 10<sup>4</sup></code>
* <code>1 <= target[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        if target.len() == 1 {
            return target[0] == 1;
        }

        let mut sum = target.iter().map(|&x| x as i64).sum::<i64>();
        let mut heap = target.iter().map(|&x| x as i64).collect::<BinaryHeap<_>>();

        while let Some(mut x) = heap.pop() {
            let y = *heap.peek().unwrap();
            let tmp = sum - x;
            x = y / tmp * tmp + x % tmp;
            if y > x {
                x += tmp;
            }
            sum = tmp + x;

            if x == 1 {
                return true;
            } else if 2 * x - sum < 1 {
                return false;
            }

            heap.push(2 * x - sum);
            sum = x;
        }

        unreachable!()
    }
}
```
