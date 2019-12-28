# 961. N-Repeated Element in Size 2N Array
In a array ```A``` of size ```2N```, there are ```N+1``` unique elements, and exactly one of these elements is repeated N times.

Return the element repeated ```N``` times.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3,3]
<strong>Output:</strong> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [2,1,2,5,3,2]
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [5,1,5,2,5,3,5,4]
<strong>Output:</strong> 5
</pre>

#### Note:
1. ```4 <= A.length <= 10000```
2. ```0 <= A[i] < 10000```
3. ```A.length``` is even

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for n in a {
            if set.contains(&n) {
                return n;
            } else {
                set.insert(n);
            }
        }

        -1
    }
}
```

### 2. Compare
```Rust
impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        for i in 1..4 {
            for j in i..a.len() {
                if a[j - i] == a[j] {
                    return a[j];
                }
            }
        }

        -1
    }
}
```
