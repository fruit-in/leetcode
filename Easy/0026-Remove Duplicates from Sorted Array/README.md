# 26. Remove Duplicates from Sorted Array
Given a sorted array *nums*, remove the duplicates **in-place** such that each element appear only *once* and return the new length.

Do not allocate extra space for another array, you must do this by **modifying the input array in-place** with O(1) extra memory.

#### Example 1:
<pre>
Given <em>nums</em> = <strong>[1,1,2]</strong>,

Your function should return length = <strong>2</strong>, with the first two elements of <em>nums</em> being <strong>1</strong> and <strong>2</strong> respectively.

It doesn't matter what you leave beyond the returned length.
</pre>

#### Example 2:
<pre>
Given <em>nums</em> = <strong>[0,0,1,1,1,2,2,3,3,4]</strong>,

Your function should return length = <strong>5</strong>, with the first five elements of <em>nums</em> being modified to <strong>0</strong>, <strong>1</strong>, <strong>2</strong>, <strong>3</strong>, and <strong>4</strong> respectively.

It doesn't matter what values are set beyond the returned length.
</pre>

#### Clarification:
Confused why the returned value is an integer but your answer is an array?

Note that the input array is passed in by **reference**, which means modification to the input array will be known to the caller as well.

Internally you can think of this:
<pre>
// <strong>nums</strong> is passed in by reference. (i.e., without making a copy)
int len = removeDuplicates(nums);

// any modification to <strong>nums</strong> in your function would be known by the caller.
// using the length returned by your function, it prints the first <strong>len</strong> elements.
for (int i = 0; i < len; i++) {
    print(nums[i]);
}
</pre>

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        i as i32 + 1
    }
}
```
