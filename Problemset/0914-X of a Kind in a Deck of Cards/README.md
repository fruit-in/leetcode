# 914. X of a Kind in a Deck of Cards
In a deck of cards, each card has an integer written on it.

Return ```true``` if and only if you can choose ```X >= 2``` such that it is possible to split the entire deck into 1 or more groups of cards, where:

* Each group has exactly ```X``` cards.
* All the cards in each group have the same integer.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3,4,4,3,2,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> Possible partition [1,1],[2,2],[3,3],[4,4]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,1,1,2,2,2,3,3]
<strong>Output:</strong> false
<strong>Explanation:</strong> No possible partition.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [1]
<strong>Output:</strong> false
<strong>Explanation:</strong> No possible partition.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> [1,1]
<strong>Output:</strong> true
<strong>Explanation:</strong> Possible partition [1,1]
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> [1,1,2,2,2,2]
<strong>Output:</strong> true
<strong>Explanation:</strong> Possible partition [1,1],[2,2],[2,2]
</pre>

#### Note:
1. ```1 <= deck.length <= 10000```
2. ```0 <= deck[i] < 10000```

## Solutions (Rust)

### 1. Brute Force
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in &deck {
            *map.entry(n).or_insert(0) += 1;
        }

        for x in (2..=(deck.len() / map.len())).filter(|x| deck.len() % x == 0) {
            if map.values().all(|v| v % x == 0) {
                return true;
            }
        }

        false
    }
}
```

### 2. GCD
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for n in deck {
            *map.entry(n).or_insert(0) += 1;
        }

        let mut x = *map.values().nth(0).unwrap();
        map.values().fold(x, |x, y| Self::gcd(x, *y)) > 1
    }

    pub fn gcd(mut x: i32, mut y: i32) -> i32 {
        while x % y != 0 {
            let tmp = x;
            x = y;
            y = tmp % y;
        }
        y
    }
}
```
