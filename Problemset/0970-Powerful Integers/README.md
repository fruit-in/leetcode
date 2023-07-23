# 970. Powerful Integers
Given two positive integers ```x``` and ```y```, an integer is *powerful* if it is equal to ```x^i + y^j``` for some integers ```i >= 0``` and ```j >= 0```.

Return a list of all *powerful* integers that have value less than or equal to ```bound```.

You may return the answer in any order.  In your answer, each value should occur at most once.

#### Example 1:
<pre>
<strong>Input:</strong> x = 2, y = 3, bound = 10
<strong>Output:</strong> [2,3,4,5,7,9,10]
<strong>Explanation:</strong>
2 = 2^0 + 3^0
3 = 2^1 + 3^0
4 = 2^0 + 3^1
5 = 2^1 + 3^1
7 = 2^2 + 3^1
9 = 2^3 + 3^0
10 = 2^0 + 3^2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> x = 3, y = 5, bound = 15
<strong>Output:</strong> [2,4,6,8,10,14]
</pre>

#### Note:
* ```1 <= x <= 100```
* ```1 <= y <= 100```
* ```0 <= bound <= 10^6```

## Solutions (Rust)

### 1. Brute Force
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set = HashSet::new();

        for i in 0..20 {
            for j in 0..20 {
                let pow_int = x.saturating_pow(i).saturating_add(y.saturating_pow(j));

                if pow_int <= bound {
                    set.insert(pow_int);
                }
            }
        }

        set.drain().collect()
    }
}
```
