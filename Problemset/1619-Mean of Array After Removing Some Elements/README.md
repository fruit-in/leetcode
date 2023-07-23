# 1619. Mean of Array After Removing Some Elements
Given an integer array `arr`, return *the mean of the remaining integers after removing the smallest `5%` and the largest `5%` of the elements*.

Answers within <code>10<sup>-5</sup></code> of the **actual answer** will be considered accepted.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]
<strong>Output:</strong> 2.00000
<strong>Explanation:</strong> After erasing the minimum and the maximum values of this array, all elements are equal to 2, so the mean is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]
<strong>Output:</strong> 4.00000
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]
<strong>Output:</strong> 4.77778
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [9,7,8,7,7,8,4,4,6,8,8,7,6,8,8,9,2,6,0,0,1,10,8,6,3,3,5,1,10,9,0,7,10,0,10,4,1,10,6,9,3,6,0,0,2,7,0,6,7,2,9,7,7,3,0,1,6,1,10,3]
<strong>Output:</strong> 5.27778
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [4,8,4,10,0,7,1,3,7,8,8,3,4,1,6,2,1,1,8,0,9,8,0,3,9,10,3,10,1,10,7,3,2,1,4,9,10,7,6,4,0,8,5,1,2,1,6,2,5,0,7,10,9,10,3,7,10,5,8,5,7,6,7,6,10,9,5,10,5,5,7,2,10,7,7,8,2,0,1,1]
<strong>Output:</strong> 5.29167
</pre>

#### Constraints:
* `20 <= arr.length <= 1000`
* `arr.length` **is a multiple** of `20`.
* <code>0 <= arr[i] <= 10<sup>5</sup></code>

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[]} arr
# @return {Float}
def trim_mean(arr)
  length = arr.length

  arr.sort!
  arr = arr[length / 20...-length / 20]

  arr.sum / (length * 0.9)
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        let len = arr.len() * 9 / 10;

        arr.sort_unstable();

        arr.iter().skip(len / 18).take(len).sum::<i32>() as f64 / (len as f64)
    }
}
```
