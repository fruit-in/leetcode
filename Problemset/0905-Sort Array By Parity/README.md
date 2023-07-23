# 905. Sort Array By Parity
Given an array ```A``` of non-negative integers, return an array consisting of all the even elements of ```A```, followed by all the odd elements of ```A```.

You may return any answer array that satisfies this condition.

#### Example 1:
<pre>
<strong>Input:</strong> [3,1,2,4]
<strong>Output:</strong> [2,4,3,1]
The outputs [4,2,3,1], [2,4,1,3], and [4,2,1,3] would also be accepted.
</pre>

#### Note:
1. ```1 <= A.length <= 5000```
2. ```0 <= A[i] <= 5000```

## Solutions (Rust)

### 1. Two Pointers
```Rust
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            while i < j && a[i] % 2 == 0 {
                i += 1;
            }
            while i < j && a[j] % 2 == 1 {
                j -= 1;
            }
            a.swap(i, j);
        }
        a
    }
}
```

### 2. One Pass
```Rust
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut even = Vec::new();
        let mut odd = Vec::new();
        for n in a {
            match n % 2 == 0 {
                true => even.push(n),
                false => odd.push(n),
            };
        }
        even.append(&mut odd);
        even
    }
}
```

### 3. Sort
```Rust
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        a.sort_unstable_by_key(|k| k % 2 == 1);
        a
    }
}
```
