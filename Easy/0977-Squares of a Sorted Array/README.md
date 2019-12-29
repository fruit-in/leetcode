# 977. Squares of a Sorted Array
Given an array of integers ```A``` sorted in non-decreasing order, return an array of the squares of each number, also in sorted non-decreasing order.

#### Example 1:
<pre>
<strong>Input:</strong> [-4,-1,0,3,10]
<strong>Output:</strong> [0,1,9,16,100]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [-7,-3,2,3,11]
<strong>Output:</strong> [4,9,9,49,121]
</pre>

#### Note:
1. ```1 <= A.length <= 10000```
2. ```-10000 <= A[i] <= 10000```
3. ```A``` is sorted in non-decreasing order.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut squares: Vec<i32> = a.iter().map(|&x| x * x).collect();
        squares.sort_unstable();
        squares
    }
}
```

### 2. Two Pointers
```Rust
impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();

        let mut p = a.binary_search(&0).unwrap_or_else(|x| x);
        let mut n = p;
        while n > 0 || p < a.len() {
            if n == 0 || (p < a.len() && a[p] < -a[n - 1]) {
                ret.push(a[p] * a[p]);
                p += 1;
            } else {
                ret.push(a[n - 1] * a[n - 1]);
                n -= 1;
            }
        }

        ret
    }
}
```
