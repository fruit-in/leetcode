# 1497. Check If Array Pairs Are Divisible by k
Given an array of integers `arr` of even length `n` and an integer `k`.

We want to divide the array into exactly `n / 2` pairs such that the sum of each pair is divisible by `k`.

Return `true` *If you can find a way to do that or* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,10,6,7,8,9], k = 5
<strong>Output:</strong> true
<strong>Explanation:</strong> Pairs are (1,9),(2,8),(3,7),(4,6) and (5,10).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,6], k = 7
<strong>Output:</strong> true
<strong>Explanation:</strong> Pairs are (1,6),(2,5) and(3,4).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,6], k = 10
<strong>Output:</strong> false
<strong>Explanation:</strong> You can try all possible pairs to see that there is no way to divide arr into 3 pairs each with sum divisible by 10.
</pre>

#### Constraints:
* `arr.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* `n` is even.
* <code>-10<sup>9</sup> <= arr[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];

        for &x in &arr {
            count[x.rem_euclid(k) as usize] += 1;
        }

        for i in 1..(k as usize + 1) / 2 {
            if count[i] != count[k as usize - i] {
                return false;
            }
        }

        k % 2 == 1 || count[k as usize / 2] % 2 == 0
    }
}
```
