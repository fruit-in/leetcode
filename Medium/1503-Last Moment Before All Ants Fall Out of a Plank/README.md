# 1503. Last Moment Before All Ants Fall Out of a Plank
We have a wooden plank of the length `n` **units**. Some ants are walking on the plank, each ant moves with speed **1 unit per second**. Some of the ants move to the **left**, the other move to the **right**.

When two ants moving in two **different** directions meet at some point, they change their directions and continue moving again. Assume changing directions doesn't take any additional time.

When an ant reaches **one end** of the plank at a time `t`, it falls out of the plank imediately.

Given an integer `n` and two integer arrays `left` and `right`, the positions of the ants moving to the left and the right. Return *the moment* when the last ant(s) fall out of the plank.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/06/17/ants.jpg)
<pre>
<strong>Input:</strong> n = 4, left = [4,3], right = [0,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> In the image above:
-The ant at index 0 is named A and going to the right.
-The ant at index 1 is named B and going to the right.
-The ant at index 3 is named C and going to the left.
-The ant at index 4 is named D and going to the left.
Note that the last moment when an ant was on the plank is t = 4 second, after that it falls imediately out of the plank. (i.e. We can say that at t = 4.0000000001, there is no ants on the plank).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/06/17/ants2.jpg)
<pre>
<strong>Input:</strong> n = 7, left = [], right = [0,1,2,3,4,5,6,7]
<strong>Output:</strong> 7
<strong>Explanation:</strong> All ants are going to the right, the ant at index 0 needs 7 seconds to fall.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/06/17/ants3.jpg)
<pre>
<strong>Input:</strong> n = 7, left = [0,1,2,3,4,5,6,7], right = []
<strong>Output:</strong> 7
<strong>Explanation:</strong> All ants are going to the left, the ant at index 7 needs 7 seconds to fall.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 9, left = [5], right = [4]
<strong>Output:</strong> 5
<strong>Explanation:</strong> At t = 1 second, both ants will be at the same intial position but with different direction.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> n = 6, left = [6], right = [0]
<strong>Output:</strong> 6
</pre>

#### Constraints:
* `1 <= n <= 10^4`
* `0 <= left.length <= n + 1`
* `0 <= left[i] <= n`
* `0 <= right.length <= n + 1`
* `0 <= right[i] <= n`
* `1 <= left.length + right.length <= n + 1`
* All values of `left` and `right` are unique, and each value can appear **only in one** of the two arrays.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer} n
# @param {Integer[]} left
# @param {Integer[]} right
# @return {Integer}
def get_last_moment(n, left, right)
  (left + right.map { |x| n - x }).max
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        left.into_iter()
            .chain(right.into_iter().map(|x| n - x))
            .max()
            .unwrap()
    }
}
```
