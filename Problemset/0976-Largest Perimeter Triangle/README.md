# 976. Largest Perimeter Triangle
Given an array ```A``` of positive lengths, return the largest perimeter of a triangle with **non-zero area**, formed from 3 of these lengths.

If it is impossible to form any triangle of non-zero area, return ```0```.

#### Example 1:
<pre>
<strong>Input:</strong> [2,1,2]
<strong>Output:</strong> 5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,1]
<strong>Output:</strong> 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [3,2,3,4]
<strong>Output:</strong> 10
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> [3,6,2,3]
<strong>Output:</strong> 8
</pre>

#### Note:
1. ```3 <= A.length <= 10000```
2. ```1 <= A[i] <= 10^6```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn largest_perimeter(a: Vec<i32>) -> i32 {
        let mut a = a;
        a.sort_unstable_by(|a, b| b.cmp(a));

        for i in 2..a.len() {
            if a[i] + a[i - 1] > a[i - 2] {
                return a[i] + a[i - 1] + a[i - 2];
            }
        }

        0
    }
}
```
