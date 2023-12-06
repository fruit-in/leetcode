# 1865. Finding Pairs With a Certain Sum
You are given two integer arrays `nums1` and `nums2`. You are tasked to implement a data structure that supports queries of two types:

1. **Add** a positive integer to an element of a given index in the array `nums2`.
2. **Count** the number of pairs `(i, j)` such that `nums1[i] + nums2[j]` equals a given value (`0 <= i < nums1.length` and `0 <= j < nums2.length`).

Implement the `FindSumPairs` class:

* `FindSumPairs(int[] nums1, int[] nums2)` Initializes the `FindSumPairs` object with two integer arrays `nums1` and `nums2`.
* `void add(int index, int val)` Adds `val` to `nums2[index]`, i.e., apply `nums2[index] += val`.
* `int count(int tot)` Returns the number of pairs `(i, j)` such that `nums1[i] + nums2[j] == tot`.

#### Example 1:
<pre>
<strong>Input:</strong>
["FindSumPairs", "count", "add", "count", "count", "add", "add", "count"]
[[[1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]], [7], [3, 2], [8], [4], [0, 1], [1, 1], [7]]
<strong>Output:</strong>
[null, 8, null, 2, 1, null, null, 11]
<strong>Explanation:</strong>
FindSumPairs findSumPairs = new FindSumPairs([1, 1, 2, 2, 2, 3], [1, 4, 5, 2, 5, 4]);
findSumPairs.count(7);  // return 8; pairs (2,2), (3,2), (4,2), (2,4), (3,4), (4,4) make 2 + 5 and pairs (5,1), (5,5) make 3 + 4
findSumPairs.add(3, 2); // now nums2 = [1,4,5,4,5,4]
findSumPairs.count(8);  // return 2; pairs (5,2), (5,4) make 3 + 5
findSumPairs.count(4);  // return 1; pair (5,0) makes 3 + 1
findSumPairs.add(0, 1); // now nums2 = [2,4,5,4,5,4]
findSumPairs.add(1, 1); // now nums2 = [2,5,5,4,5,4]
findSumPairs.count(7);  // return 11; pairs (2,1), (2,2), (2,4), (3,1), (3,2), (3,4), (4,1), (4,2), (4,4) make 2 + 5 and pairs (5,3), (5,5) make 3 + 4
</pre>

#### Constraints:
* `1 <= nums1.length <= 1000`
* <code>1 <= nums2.length <= 10<sup>5</sup></code>
* <code>1 <= nums1[i] <= 10<sup>9</sup></code>
* <code>1 <= nums2[i] <= 10<sup>5</sup></code>
* `0 <= index < nums2.length`
* <code>1 <= val <= 10<sup>5</sup></code>
* <code>1 <= tot <= 10<sup>9</sup></code>
* At most `1000` calls are made to `add` and `count` **each**.

## Solutions (Rust)

### 1. Solution
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
