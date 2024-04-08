# 1798. Maximum Number of Consecutive Values You Can Make
You are given an integer array `coins` of length `n` which represents the `n` coins that you own. The value of the <code>i<sup>th</sup></code> coin is `coins[i]`. You can **make** some value `x` if you can choose some of your `n` coins such that their values sum up to `x`.

Return the *maximum number of consecutive integer values that you **can make** with your coins **starting** from and **including*** `0`.

Note that you may have multiple coins of the same value.

#### Example 1:
<pre>
<strong>Input:</strong> coins = [1,3]
<strong>Output:</strong> 2
<strong>Explanation:</strong> You can make the following values:
- 0: take []
- 1: take [1]
You can make 2 consecutive integer values starting from 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> coins = [1,1,1,4]
<strong>Output:</strong> 8
<strong>Explanation:</strong> You can make the following values:
- 0: take []
- 1: take [1]
- 2: take [1,1]
- 3: take [1,1,1]
- 4: take [4]
- 5: take [4,1]
- 6: take [4,1,1]
- 7: take [4,1,1,1]
You can make 8 consecutive integer values starting from 0.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,4,10,3,1]
<strong>Output:</strong> 20
</pre>

#### Constraints:
* `coins.length == n`
* <code>1 <= n <= 4 * 10<sup>4</sup></code>
* <code>1 <= coins[i] <= 4 * 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_maximum_consecutive(mut coins: Vec<i32>) -> i32 {
        let mut ret = 1;

        coins.sort_unstable();

        for &coin in &coins {
            if coin > ret {
                break;
            }

            ret += coin;
        }

        ret
    }
}
```
