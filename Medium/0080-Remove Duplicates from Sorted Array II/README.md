# 80. Remove Duplicates from Sorted Array II
Given a sorted array *nums*, remove the duplicates **[in-place](https://en.wikipedia.org/wiki/In-place_algorithm)** such that duplicates appeared at most *twice* and return the new length.

Do not allocate extra space for another array, you must do this by **modifying the input array [in-place](https://en.wikipedia.org/wiki/In-place_algorithm)** with O(1) extra memory.

#### Example 1:
<pre>
Given <i>nums</i> = <strong>[1,1,1,2,2,3]</strong>,

Your function should return length = <b>5</b>, with the first five elements of <i>nums</i> being <b>1, 1, 2, 2</b> and <b>3</b> respectively.

It doesn't matter what you leave beyond the returned length.
</pre>

#### Example 2:
<pre>
Given <i>nums</i> = <b>[0,0,1,1,1,1,2,3,3]</b>,

Your function should return length = <b>7</b>, with the first seven elements of <i>nums</i> being modified to <b>0, 0, 1, 1, 2, 3</b> and <b>3</b> respectively.

It doesn't matter what values are set beyond the returned length.
</pre>

#### Clarification:
Confused why the returned value is an integer but your answer is an array?

Note that the input array is passed in by **reference**, which means modification to the input array will be known to the caller as well.

Internally you can think of this:
<pre>
// <b>nums</b> is passed in by reference. (i.e., without making a copy)
int len = removeDuplicates(nums);

// any modification to <b>nums</b> in your function would be known by the caller.
// using the length returned by your function, it prints the first <b>len</b> elements.
for (int i = 0; i < len; i++) {
    print(nums[i]);
}
</pre>

## Solutions (Rust)

### 1. Remove Duplicates
```Rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 2;

        while i < nums.len() {
            if nums[i] == nums[i - 2] {
                nums.remove(i);
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}
```

### 2. Two Pointers
```Rust
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;

        for j in 0..nums.len() {
            if i < 2 || nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }

        i as i32
    }
}
```
