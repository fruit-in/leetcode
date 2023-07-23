# 202. Happy Number
Write an algorithm to determine if a number is "happy".

A happy number is a number defined by the following process: Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.

#### Example:
<pre>
<strong>Input:</strong> 19
<strong>Output:</strong> true
<strong>Explanation:</strong>
1<sup>2</sup> + 9<sup>2</sup> = 82
8<sup>2</sup> + 2<sup>2</sup> = 68
6<sup>2</sup> + 8<sup>2</sup> = 100
1<sup>2</sup> + 0<sup>2</sup> + 0<sup>2</sup> = 1
</pre>

## Solutions (Ruby)

### 1. Set
```Ruby
# @param {Integer} n
# @return {Boolean}
def is_happy(n)
    set = Set.new

    while not set.include?(n)
        set.add(n)
        new_n = 0
        while n > 0
            new_n += (n % 10) ** 2
            n /= 10
        end
        n = new_n
    end

    return n == 1
end
```

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut set = HashSet::new();
        let mut n = n;
        let mut new_n: i32;
        while !set.contains(&n) {
            set.insert(n);
            new_n = 0;
            while n > 0 {
                new_n += (n % 10).pow(2);
                n /= 10;
            }
            n = new_n;
        }
        n == 1
    }
}
```
