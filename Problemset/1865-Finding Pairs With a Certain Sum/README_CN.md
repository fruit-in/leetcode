# 1865. 找出和为指定值的下标对
给你两个整数数组 `nums1` 和 `nums2` ，请你实现一个支持下述两类查询的数据结构：

1. **累加** ，将一个正整数加到 `nums2` 中指定下标对应元素上。
2. **计数** ，统计满足 `nums1[i] + nums2[j]` 等于指定值的下标对 `(i, j)` 数目（`0 <= i < nums1.length` 且 `0 <= j < nums2.length`）。

实现 `FindSumPairs` 类：

* `FindSumPairs(int[] nums1, int[] nums2)` 使用整数数组 `nums1` 和 `nums2` 初始化 `FindSumPairs` 对象。
* `void add(int index, int val)` 将 `val` 加到 `nums2[index]` 上，即，执行 `nums2[index] += val` 。
* `int count(int tot)` 返回满足 `nums1[i] + nums2[j] == tot` 的下标对 `(i, j)` 数目。

#### 示例 1:
<pre>
<strong>输入:</strong>
["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"]
[[[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]], [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]
<strong>输出:</strong>
[null, 8, null, 2, 1, null, null, 11]
<strong>解释:</strong>
FindSumPairs findSumPairs = new FindSumPairs([1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]);
findSumPairs.count(7);  // 返回 8 ; 下标对 (2,2), (3,2), (4,2), (2,4), (3,4), (4,4) 满足 2 + 5 = 7 ，下标对 (5,1), (5,5) 满足 3 + 4 = 7
findSumPairs.add(3, 2); // 此时 nums2 = [1,4,5,4,5,4]
findSumPairs.count(8);  // 返回 2 ；下标对 (5,2), (5,4) 满足 3 + 5 = 8
findSumPairs.count(4);  // 返回 1 ；下标对 (5,0) 满足 3 + 1 = 4
findSumPairs.add(0, 1); // 此时 nums2 = [2,4,5,4,5,4]
findSumPairs.add(1, 1); // 此时 nums2 = [2,5,5,4,5,4]
findSumPairs.count(7);  // 返回 11 ；下标对 (2,1), (2,2), (2,4), (3,1), (3,2), (3,4), (4,1), (4,2), (4,4) 满足 2 + 5 = 7 ，下标对 (5,3), (5,5) 满足 3 + 4 = 7
</pre>

#### 提示:
* `1 <= nums1.length <= 1000`
* <code>1 <= nums2.length <= 10<sup>5</sup></code>
* <code>1 <= nums1[i] <= 10<sup>9</sup></code>
* <code>1 <= nums2[i] <= 10<sup>5</sup></code>
* `0 <= index < nums2.length`
* <code>1 <= val <= 10<sup>5</sup></code>
* <code>1 <= tot <= 10<sup>9</sup></code>
* 最多调用 `add` 和 `count` 函数各 `1000` 次

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

struct FindSumPairs {
    nums2: Vec<i32>,
    count1: HashMap<i32, i32>,
    count2: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {
    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut count1 = HashMap::new();
        let mut count2 = HashMap::new();

        for &num in &nums1 {
            *count1.entry(num).or_insert(0) += 1;
        }
        for &num in &nums2 {
            *count2.entry(num).or_insert(0) += 1;
        }

        Self {
            nums2,
            count1,
            count2,
        }
    }

    fn add(&mut self, index: i32, val: i32) {
        *self.count2.get_mut(&self.nums2[index as usize]).unwrap() -= 1;
        self.nums2[index as usize] += val;
        *self.count2.entry(self.nums2[index as usize]).or_insert(0) += 1;
    }

    fn count(&self, tot: i32) -> i32 {
        self.count1
            .iter()
            .map(|(&k, &v)| v * *self.count2.get(&(tot - k)).unwrap_or(&0))
            .sum()
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
```
