# 1482. Minimum Number of Days to Make m Bouquets
You are given an integer array `bloomDay`, an integer `m` and an integer `k`.

You want to make `m` bouquets. To make a bouquet, you need to use `k` **adjacent flowers** from the garden.

The garden consists of `n` flowers, the <code>i<sup>th</sup></code> flower will bloom in the `bloomDay[i]` and then can be used in **exactly one** bouquet.

Return *the minimum number of days you need to wait to be able to make* `m` *bouquets from the garden*. If it is impossible to make m bouquets return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> bloomDay = [1,10,3,10,2], m = 3, k = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> Let us see what happened in the first three days. x means flower bloomed and _ means flower did not bloom in the garden.
We need 3 bouquets each should contain 1 flower.
After day 1: [x, _, _, _, _]   // we can only make one bouquet.
After day 2: [x, _, _, _, x]   // we can only make two bouquets.
After day 3: [x, _, x, _, x]   // we can make 3 bouquets. The answer is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> bloomDay = [1,10,3,10,2], m = 3, k = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong> We need 3 bouquets each has 2 flowers, that means we need 6 flowers. We only have 5 flowers so it is impossible to get the needed bouquets and we return -1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> bloomDay = [7,7,7,7,12,7,7], m = 2, k = 3
<strong>Output:</strong> 12
<strong>Explanation:</strong> We need 2 bouquets each should have 3 flowers.
Here is the garden after the 7 and 12 days:
After day 7: [x, x, x, x, _, x, x]
We can make one bouquet of the first three flowers that bloomed. We cannot make another bouquet from the last three flowers that bloomed because they are not adjacent.
After day 12: [x, x, x, x, x, x, x]
It is obvious that we can make two bouquets in different ways.
</pre>

#### Constraints:
* `bloomDay.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= bloomDay[i] <= 10<sup>9</sup></code>
* <code>1 <= m <= 10<sup>6</sup></code>
* `1 <= k <= n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        if m as i64 * k as i64 > bloom_day.len() as i64 {
            return -1;
        }

        let mut low = *bloom_day.iter().min().unwrap();
        let mut high = *bloom_day.iter().max().unwrap();

        while low < high {
            let day = (low + high) / 2;
            let mut bouquets = 0;
            let mut i = 0;
            let mut j = 0;

            while j < bloom_day.len() {
                if bloom_day[i] > day && bloom_day[j] <= day {
                    i = j;
                }

                if bloom_day[j] <= day && j - i + 1 == k as usize {
                    bouquets += 1;
                    i = j + 1;
                } else if bloom_day[j] > day {
                    i = j + 1;
                }

                j += 1;
            }

            if bouquets < m {
                low = day + 1;
            } else {
                high = day;
            }
        }

        high
    }
}
```
