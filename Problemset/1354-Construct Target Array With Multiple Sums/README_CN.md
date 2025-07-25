# 1354. 多次求和构造目标数组
给你一个整数数组 `target` 。一开始，你有一个数组 `A` ，它的所有元素均为 1 ，你可以执行以下操作：
* 令 `x` 为你数组里所有元素的和
* 选择满足 `0 <= i < target.size` 的任意下标 `i` ，并让 `A` 数组里下标为 `i` 处的值为 `x` 。
* 你可以重复该过程任意次

如果能从 `A` 开始构造出目标数组 `target` ，请你返回 True ，否则返回 False 。

#### 示例 1:
<pre>
<strong>输入:</strong> target = [9,3,5]
<strong>输出:</strong> true
<strong>解释:</strong> 从 [1, 1, 1] 开始
[1, 1, 1], 和为 3 ，选择下标 1
[1, 3, 1], 和为 5， 选择下标 2
[1, 3, 5], 和为 9， 选择下标 0
[9, 3, 5] 完成
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = [1,1,1,2]
<strong>输出:</strong> false
<strong>解释:</strong> 不可能从 [1,1,1,1] 出发构造目标数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> target = [8,5]
<strong>输出:</strong> true
</pre>

#### 提示:
* `N == target.length`
* <code>1 <= target.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= target[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
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
