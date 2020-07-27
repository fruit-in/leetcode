# 179. Largest Number
Given a list of non negative integers, arrange them such that they form the largest number.

#### Example 1:
<pre>
<strong>Input:</strong> [10,2]
<strong>Output:</strong> "210"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [3,30,34,5,9]
<strong>Output:</strong> "9534330"
</pre>

**Note:** The result may be very large, so you need to return a string instead of an integer.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|num| num.to_string()).collect::<Vec<_>>();
        nums.sort_unstable_by(|a, b| (b.to_owned() + a).cmp(&(a.to_owned() + b)));

        if nums.len() > 0 && nums[0] == 0.to_string() {
            return 0.to_string();
        }
        nums.concat()
    }
}
```
