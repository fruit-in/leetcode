# 1220. Count Vowels Permutation
Given an integer ```n```, your task is to count how many strings of length ```n``` can be formed under the following rules:
* Each character is a lower case vowel (```'a'```, ```'e'```, ```'i'```, ```'o'```, ```'u'```)
* Each vowel ```'a'``` may only be followed by an ```'e'```.
* Each vowel ```'e'``` may only be followed by an ```'a'``` or an ```'i'```.
* Each vowel ```'i'``` **may not** be followed by another ```'i'```.
* Each vowel ```'o'``` may only be followed by an ```'i'``` or a ```'u'```.
* Each vowel ```'u'``` may only be followed by an ```'a'```.

Since the answer may be too large, return it modulo ```10^9 + 7.```

#### Example 1:
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 5
<strong>Explanation:</strong> All possible strings are: "a", "e", "i" , "o" and "u".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 10
<strong>Explanation:</strong> All possible strings are: "ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" and "ua".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 5
<strong>Output:</strong> 68
</pre>

#### Constraints:
* ```1 <= n <= 2 * 10^4```

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut end = vec![1 as u32; 5];

        for _ in 1..n {
            let mut tmp = Vec::new();
            tmp.push((end[1] + end[2] + end[4]) % 1_000_000_007);
            tmp.push((end[0] + end[2]) % 1_000_000_007);
            tmp.push((end[1] + end[3]) % 1_000_000_007);
            tmp.push((end[2]) % 1_000_000_007);
            tmp.push((end[2] + end[3]) % 1_000_000_007);
            end = tmp;
        }

        (end.iter().sum::<u32>() % 1_000_000_007) as i32
    }
}
```
