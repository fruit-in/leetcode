# 27. Remove Element
Given an array *nums* and a value *val*, remove all instances of that value **in-place** and return the new length.

Do not allocate extra space for another array, you must do this by **modifying the input array in-place** with O(1) extra memory.

The order of elements can be changed. It doesn't matter what you leave beyond the new length.

#### Example 1:
<pre>
Given <em>nums</em> = <strong>[3,2,2,3]</strong>, <em>val</em> = <strong>3</strong>,

Your function should return length = <strong>2</strong>, with the first two elements of <em>nums</em> being <strong>2</strong>.

It doesn't matter what you leave beyond the returned length.
</pre>

#### Example 2:
<pre>
Given <em>nums</em> = <strong>[0,1,2,2,3,0,4,2]</strong>, <em>val</em> = <strong>2</strong>,

Your function should return length = <strong>5</strong>, with the first five elements of <em>nums</em> containing <strong>0</strong>, <strong>1</strong>, <strong>3</strong>, <strong>0</strong>, and <strong>4</strong>.

Note that the order of those five elements can be arbitrary.

It doesn't matter what values are set beyond the returned length.
</pre>

#### Clarification:
Confused why the returned value is an integer but your answer is an array?

Note that the input array is passed in by **reference**, which means modification to the input array will be known to the caller as well.

Internally you can think of this:

<pre>
// <strong>nums</strong> is passed in by reference. (i.e., without making a copy)
int len = removeElement(nums, val);

// any modification to <strong>nums</strong> in your function would be known by the caller.
// using the length returned by your function, it prints the first <strong>len</strong> elements.
for (int i = 0; i < len; i++) {
    print(nums[i]);
}
</pre>

## Solutions (Ruby)

### 1. Two Pointers
```Ruby
# @param {Integer[]} nums
# @param {Integer} val
# @return {Integer}
def remove_element(nums, val)
    ret = 0

    for i in 0...nums.length
        if nums[i] != val
            nums[ret] = nums[i]
            ret += 1
        end
    end

    return ret
end
```

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[ans] = nums[i];
                ans += 1;
            }
        }
        ans as i32
    }
}
```
