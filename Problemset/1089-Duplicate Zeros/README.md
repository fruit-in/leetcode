# 1089. Duplicate Zeros
Given a fixed length array ```arr``` of integers, duplicate each occurrence of zero, shifting the remaining elements to the right.

Note that elements beyond the length of the original array are not written.

Do the above modifications to the input array **in place**, do not return anything from your function.

#### Example 1:
<pre>
<strong>Input:</strong> [1,0,2,3,0,4,5,0]
<strong>Output:</strong> null
<strong>Explanation:</strong> After calling your function, the <strong>input</strong> array is modified to: [1,0,0,2,3,0,0,4]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,3]
<strong>Output:</strong> null
<strong>Explanation:</strong> After calling your function, the <strong>input</strong> array is modified to: [1,2,3]
</pre>

#### Note:
1. ```1 <= arr.length <= 10000```
2. ```0 <= arr[i] <= 9```

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                for j in ((i + 1)..arr.len()).rev() {
                    arr[j] = arr[j - 1];
                }
                i += 1;
            }
            i += 1;
        }
    }
}
```

### 2. Count Zeroes
```Rust
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut zeroes = 0;

        for i in 0..arr.len() {
            if i + zeroes / 2 >= arr.len() - 1 {
                if arr[i] == 0 && i + zeroes / 2 == arr.len() - 1 {
                    zeroes += 1;
                }
                break;
            }

            if arr[i] == 0 {
                zeroes += 2;
            }
        }

        for i in (0..arr.len()).rev() {
            arr[i] = arr[i - zeroes / 2];
            if arr[i] == 0 {
                zeroes -= 1;
            }
        }
    }
}
```
