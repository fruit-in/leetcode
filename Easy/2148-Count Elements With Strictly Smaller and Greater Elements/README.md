# 2148. Count Elements With Strictly Smaller and Greater Elements
Given an integer array `nums`, return *the number of elements that have **both** a strictly smaller and a strictly greater element appear in* `nums`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [11,7,2,15]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The element 7 has the element 2 strictly smaller than it and the element 11 strictly greater than it.
Element 11 has element 7 strictly smaller than it and element 15 strictly greater than it.
In total there are 2 elements having both a strictly smaller and a strictly greater element appear in nums.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-3,3,3,90]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The element 3 has the element -3 strictly smaller than it and the element 90 strictly greater than it.
Since there are two elements with the value 3, in total there are 2 elements having both a strictly smaller and a strictly greater element appear in nums.
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let max = nums.iter().max().unwrap();

        nums.iter().filter(|&num| num != min && num != max).count() as i32
    }
}
```
