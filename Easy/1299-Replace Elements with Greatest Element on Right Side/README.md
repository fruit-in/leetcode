# 1299. Replace Elements with Greatest Element on Right Side
Given an array ```arr```, replace every element in that array with the greatest element among the elements to its right, and replace the last element with ```-1```.

After doing so, return the array.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [17,18,5,4,6,1]
<strong>Output:</strong> [18,6,6,6,1,-1]
</pre>

#### Constraints:
* ```1 <= arr.length <= 10^4```
* ```1 <= arr[i] <= 10^5```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut greatest = -1;

        for i in (0..arr.len()).rev() {
            let tmp = greatest;
            greatest = greatest.max(arr[i]);
            arr[i] = tmp;
        }

        arr
    }
}
```
