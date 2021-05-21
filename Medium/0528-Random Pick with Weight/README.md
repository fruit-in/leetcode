# 528. Random Pick with Weight
You are given an array of positive integers `w` where `w[i]` describes the weight of <code>i<sup>th</sup></code> index (0-indexed).

We need to call the function `pickIndex()` which **randomly** returns an integer in the range `[0, w.length - 1]`. `pickIndex()` should return the integer proportional to its weight in the `w` array. For example, for `w = [1, 3]`, the probability of picking the index `0` is `1 / (1 + 3) = 0.25` (i.e 25%) while the probability of picking the index `1` is `3 / (1 + 3) = 0.75` (i.e 75%).

More formally, the probability of picking index `i` is `w[i] / sum(w)`.

#### Example 1:
<pre>
<strong>Input:</strong>
["Solution","pickIndex"]
[[[1]],[]]
<strong>Output:</strong>
[null,0]
<strong>Explanation:</strong>
Solution solution = new Solution([1]);
solution.pickIndex(); // return 0. Since there is only one single element on the array the only option is to return the first element.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
["Solution","pickIndex","pickIndex","pickIndex","pickIndex","pickIndex"]
[[[1,3]],[],[],[],[],[]]
<strong>Output:</strong>
[null,1,1,1,1,0]
<strong>Explanation:</strong>
Solution solution = new Solution([1, 3]);
solution.pickIndex(); // return 1. It's returning the second element (index = 1) that has probability of 3/4.
solution.pickIndex(); // return 1
solution.pickIndex(); // return 1
solution.pickIndex(); // return 1
solution.pickIndex(); // return 0. It's returning the first element (index = 0) that has probability of 1/4.

Since this is a randomization problem, multiple answers are allowed so the following outputs can be considered correct :
[null,1,1,1,1,0]
[null,1,1,1,1,1]
[null,1,1,1,0,0]
[null,1,1,1,0,1]
[null,1,0,1,0,0]
......
and so on.
</pre>

#### Constraints:
* `1 <= w.length <= 10000`
* `1 <= w[i] <= 10^5`
* `pickIndex` will be called at most `10000` times.

## Solutions (Rust)

### 1. Binary Search
```Rust
use rand::{thread_rng, Rng};

struct Solution {
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(mut w: Vec<i32>) -> Self {
        for i in 1..w.len() {
            w[i] += w[i - 1];
        }

        Self { prefix_sum: w }
    }

    fn pick_index(&self) -> i32 {
        let x = thread_rng().gen_range(1, self.prefix_sum.last().unwrap() + 1);

        match self.prefix_sum.binary_search(&x) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
```
