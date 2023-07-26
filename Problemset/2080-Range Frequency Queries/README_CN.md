# 2080. 区间内查询数字的频率
请你设计一个数据结构，它能求出给定子数组内一个给定值的 **频率** 。

子数组中一个值的 **频率** 指的是这个子数组中这个值的出现次数。

请你实现 `RangeFreqQuery` 类：

* `RangeFreqQuery(int[] arr)` 用下标从 **0** 开始的整数数组 `arr` 构造一个类的实例。
* `int query(int left, int right, int value)` 返回子数组 `arr[left...right]` 中 `value` 的 **频率** 。

一个 **子数组** 指的是数组中一段连续的元素。`arr[left...right]` 指的是 `nums` 中包含下标 `left` 和 `right` **在内** 的中间一段连续元素。

#### 示例 1:
<pre>
<strong>输入:</strong>
["RangeFreqQuery", "query", "query"]
[[[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]], [1, 2, 4], [0, 11, 33]]
<strong>输出:</strong>
[null, 1, 2]
<strong>解释:</strong>
RangeFreqQuery rangeFreqQuery = new RangeFreqQuery([12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
rangeFreqQuery.query(1, 2, 4); // 返回 1 。4 在子数组 [33, 4] 中出现 1 次。
rangeFreqQuery.query(0, 11, 33); // 返回 2 。33 在整个子数组中出现 2 次。
</pre>

#### 提示:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>1 <= arr[i], value <= 10<sup>4</sup></code>
* `0 <= left <= right < arr.length`
* 调用 `query` 不超过 <code>10<sup>5</sup></code> 次。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

struct RangeFreqQuery {
    indices: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut indices = HashMap::new();

        for i in 0..arr.len() {
            indices
                .entry(arr[i])
                .and_modify(|v: &mut Vec<i32>| v.push(i as i32))
                .or_insert(vec![i as i32]);
        }

        Self { indices }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if !self.indices.contains_key(&value) {
            return 0;
        }

        let indices = self.indices.get(&value).unwrap();
        let left = match indices.binary_search(&left) {
            Ok(i) | Err(i) => i as i32,
        };
        let right = match indices.binary_search(&right) {
            Ok(i) => i as i32,
            Err(i) => i as i32 - 1,
        };

        right - left + 1
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
```
