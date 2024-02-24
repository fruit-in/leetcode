# 514. Freedom Trail
In the video game Fallout 4, the quest **"Road to Freedom"** requires players to reach a metal dial called the **"Freedom Trail Ring"** and use the dial to spell a specific keyword to open the door.

Given a string `ring` that represents the code engraved on the outer ring and another string `key` that represents the keyword that needs to be spelled, return *the minimum number of steps to spell all the characters in the keyword*.

Initially, the first character of the ring is aligned at the `"12:00"` direction. You should spell all the characters in `key` one by one by rotating `ring` clockwise or anticlockwise to make each character of the string key aligned at the `"12:00"` direction and then by pressing the center button.

At the stage of rotating the ring to spell the key character `key[i]`:

1. You can rotate the ring clockwise or anticlockwise by one place, which counts as **one step**. The final purpose of the rotation is to align one of `ring`'s characters at the `"12:00"` direction, where this character must equal `key[i]`.
2. If the character `key[i]` has been aligned at the `"12:00"` direction, press the center button to spell, which also counts as **one step**. After the pressing, you could begin to spell the next character in the key (next stage). Otherwise, you have finished all the spelling.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/10/22/ring.jpg)
<pre>
<strong>Input:</strong> https://assets.leetcode.com/uploads/2018/10/22/ring.jpg
<strong>Output:</strong> 4
<strong>Explanation:</strong>
For the first key character 'g', since it is already in place, we just need 1 step to spell this character.
For the second key character 'd', we need to rotate the ring "godding" anticlockwise by two steps to make it become "ddinggo".
Also, we need 1 more step for spelling.
So the final output is 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ring = "godding", key = "godding"
<strong>Output:</strong> 13
</pre>

#### Constraints:
* `1 <= ring.length, key.length <= 100`
* `ring` and `key` consist of only lower case English letters.
* It is guaranteed that `key` could always be spelled by rotating `ring`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let ring = ring.as_bytes();
        let key = key.as_bytes();
        let mut dp = vec![None; n];
        dp[0] = Some(0);

        for i in 0..key.len() {
            let mut tmp: Vec<Option<i32>> = vec![None; n];

            for j in 0..n {
                if ring[j] != key[i] {
                    continue;
                }

                for k in 0..n {
                    if let Some(x) = dp[k] {
                        let y = tmp[j].unwrap_or(i32::MAX);
                        let a = j.min(k) as i32;
                        let b = j.max(k) as i32;
                        tmp[j] = Some(y.min(x + b - a + 1).min(x + n as i32 - b + a + 1));
                    }
                }
            }

            dp = tmp;
        }

        dp.iter().map(|&x| x.unwrap_or(i32::MAX)).min().unwrap()
    }
}
```
