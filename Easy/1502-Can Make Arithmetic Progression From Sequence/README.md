# 1502. Can Make Arithmetic Progression From Sequence
Given an array of numbers `arr`. A sequence of numbers is called an arithmetic progression if the difference between any two consecutive elements is the same.

Return `true` if the array can be rearranged to form an arithmetic progression, otherwise, return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,5,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can reorder the elements as [1,3,5] or [5,3,1] with differences 2 and -2 respectively, between each consecutive elements.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2,4]
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no way to reorder the elements to obtain an arithmetic progression.
</pre>

#### Constraints:
* `2 <= arr.length <= 1000`
* `-10^6 <= arr[i] <= 10^6`

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[]} arr
# @return {Boolean}
def can_make_arithmetic_progression(arr)
    arr.sort!

    for i in 2...arr.length
        return false if arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2]
    end

    return true
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        arr.sort_unstable();

        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2] {
                return false;
            }
        }

        true
    }
}
```
