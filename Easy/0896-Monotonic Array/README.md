# 896. Monotonic Array
An array is *monotonic* if it is either monotone increasing or monotone decreasing.

An array ```A``` is monotone increasing if for all ```i <= j```, ```A[i] <= A[j]```.  An array ```A``` is monotone decreasing if for all ```i <= j```, ```A[i] >= A[j]```.

Return ```true``` if and only if the given array ```A``` is monotonic.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,2,3]
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [6,5,4,4]
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [1,3,2]
<strong>Output:</strong> false
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> [1,2,4,5]
<strong>Output:</strong> true
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> [1,1,1]
<strong>Output:</strong> true
</pre>

#### Note:
1. ```1 <= A.length <= 50000```
2. ```-100000 <= A[i] <= 100000```

## Solutions (Rust)

### 1. Linear Scan
```Rust
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut flag = 0;

        for i in 1..a.len() {
            if a[i - 1] != a[i] {
                if flag * (a[i - 1] - a[i]) >= 0 {
                    flag = a[i - 1] - a[i];
                } else {
                    return false;
                }
            }
        }

        true
    }
}
```

### 2. Sort
```Rust
impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut sort_a = a.clone();
        sort_a.sort_unstable();

        if sort_a == a {
            true
        } else {
            sort_a.reverse();
            sort_a == a
        }
    }
}
```
