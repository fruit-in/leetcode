# 605. Can Place Flowers
Suppose you have a long flowerbed in which some of the plots are planted and some are not. However, flowers cannot be planted in adjacent plots - they would compete for water and both would die.

Given a flowerbed (represented as an array containing 0 and 1, where 0 means empty and 1 means not empty), and a number **n**, return if **n** new flowers can be planted in it without violating the no-adjacent-flowers rule.

#### Example 1:
<pre>
<strong>Input:</strong> flowerbed = [1,0,0,0,1], n = 1
<strong>Output:</strong> True
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> flowerbed = [1,0,0,0,1], n = 2
<strong>Output:</strong> False
</pre>

#### Note:
1. The input array won't violate no-adjacent-flowers rule.
2. The input array size is in the range of [1, 20000].
3. **n** is a non-negative integer which won't exceed the input array size.

## Solutions (Rust)

### 1. Count Continuous Zeroes
```Rust
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut zeroes = 1;

        for i in flowerbed {
            if i == 0 {
                zeroes += 1;
            } else {
                n -= (zeroes - 1) / 2;
                zeroes = 0;
            }
        }

        zeroes += 1;
        n -= (zeroes - 1) / 2;

        n <= 0
    }
}
```

### 2. Simulation
```Rust
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut flowerbed = flowerbed;
        flowerbed.insert(0, 0);
        flowerbed.push(0);

        for i in 1..(flowerbed.len() - 1) {
            if flowerbed[i - 1] + flowerbed[i] + flowerbed[i + 1] == 0 {
                flowerbed[i] = 1;
                n -= 1;
            }
        }

        n <= 0
    }
}
```
