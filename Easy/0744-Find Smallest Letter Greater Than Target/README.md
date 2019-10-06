# 744. Find Smallest Letter Greater Than Target
Given a list of sorted characters ```letters``` containing only lowercase letters, and given a ```target``` letter target, find the smallest element in the list that is larger than the given target. 

Letters also wrap around. For example, if the target is ```target = 'z'``` and ```letters = ['a', 'b']```, the answer is ```'a'```.

#### Examples:
<pre>
<strong>Input:</strong>
letters = ["c", "f", "j"]
target = "a"
<strong>Output:</strong> "c"

<strong>Input:</strong>
letters = ["c", "f", "j"]
target = "c"
<strong>Output:</strong> "f"

<strong>Input:</strong>
letters = ["c", "f", "j"]
target = "d"
<strong>Output:</strong> "f"

<strong>Input:</strong>
letters = ["c", "f", "j"]
target = "g"
<strong>Output:</strong> "j"

<strong>Input:</strong>
letters = ["c", "f", "j"]
target = "j"
<strong>Output:</strong> "c"

<strong>Input:</strong>
letters = ["c", "f", "j"]
target = "k"
<strong>Output:</strong> "c"
</pre>

#### Note:
1. ```letters``` has a length in range ```[2, 10000]```.
2. ```letters``` consists of lowercase letters, and contains at least 2 unique letters.
3. ```target``` is a lowercase letter.

## Solutions (Rust)

### 1. Linear Scan
```Rust
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &ch in &letters {
            if ch > target {
                return ch;
            }
        }
        letters[0]
    }
}
```

### 2. Binary Search
```Rust
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        match letters.binary_search(&target) {
            Ok(i) => letters[(i + 1) % letters.len()],
            Err(i) => letters[i % letters.len()],
        }
    }
}
```
