# 1387. Sort Integers by The Power Value
The power of an integer ```x``` is defined as the number of steps needed to transform ```x``` into ```1``` using the following steps:
* if ```x``` is even then ```x = x / 2```
* if ```x``` is odd then ```x = 3 * x + 1```

For example, the power of x = 3 is 7 because 3 needs 7 steps to become 1 (3 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1).

Given three integers ```lo```, ```hi``` and ```k```. The task is to sort all integers in the interval ```[lo, hi]``` by the power value in **ascending order**, if two or more integers have **the same** power value sort them by **ascending order**.

Return the ```k-th``` integer in the range ```[lo, hi]``` sorted by the power value.

Notice that for any integer ```x``` ```(lo <= x <= hi)``` it is **guaranteed** that ```x``` will transform into ```1``` using these steps and that the power of ```x``` is will **fit** in 32 bit signed integer.

#### Example 1:
<pre>
<strong>Input:</strong> lo = 12, hi = 15, k = 2
<strong>Output:</strong> 13
<strong>Explanation:</strong> The power of 12 is 9 (12 --> 6 --> 3 --> 10 --> 5 --> 16 --> 8 --> 4 --> 2 --> 1)
The power of 13 is 9
The power of 14 is 17
The power of 15 is 17
The interval sorted by the power value [12,13,14,15]. For k = 2 answer is the second element which is 13.
Notice that 12 and 13 have the same power value and we sorted them in ascending order. Same for 14 and 15.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> lo = 1, hi = 1, k = 1
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> lo = 7, hi = 11, k = 4
<strong>Output:</strong> 7
<strong>Explanation:</strong> The power array corresponding to the interval [7, 8, 9, 10, 11] is [16, 3, 19, 6, 14].
The interval sorted by power is [8, 10, 11, 7, 9].
The fourth number in the sorted array is 7.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> lo = 10, hi = 20, k = 5
<strong>Output:</strong> 13
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> lo = 1, hi = 1000, k = 777
<strong>Output:</strong> 570
</pre>

#### Constraints:
* ```1 <= lo <= hi <= 1000```
* ```1 <= k <= hi - lo + 1```

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut arr = (lo..=hi).collect::<Vec<i32>>();
        let mut stack = arr.clone();
        let mut power = HashMap::new();
        power.insert(1, 0);

        while let Some(x) = stack.pop() {
            if !power.contains_key(&x) {
                let y = match x % 2 {
                    0 => x / 2,
                    _ => 3 * x + 1,
                };
                match power.get(&y) {
                    Some(z) => { power.insert(x, z + 1); },
                    None => {
                        stack.push(x);
                        stack.push(y);
                    },
                }
            }
        }

        arr.sort_by_key(|x| *power.get(&x).unwrap());

        arr[k as usize - 1]
    }
}
```
