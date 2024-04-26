# 810. Chalkboard XOR Game
You are given an array of integers `nums` represents the numbers written on a chalkboard.

Alice and Bob take turns erasing exactly one number from the chalkboard, with Alice starting first. If erasing a number causes the bitwise XOR of all the elements of the chalkboard to become `0`, then that player loses. The bitwise XOR of one element is that element itself, and the bitwise XOR of no elements is `0`.

Also, if any player starts their turn with the bitwise XOR of all the elements of the chalkboard equal to `0`, then that player wins.

Return `true` *if and only if Alice wins the game, assuming both players play optimally*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,1,2]
<strong>Output:</strong> false
<strong>Explanation:</strong>
Alice has two choices: erase 1 or erase 2.
If she erases 1, the nums array becomes [1, 2]. The bitwise XOR of all the elements of the chalkboard is 1 XOR 2 = 3. Now Bob can remove any element he wants, because Alice will be the one to erase the last element and she will lose.
If Alice erases 2 first, now nums become [1, 1]. The bitwise XOR of all the elements of the chalkboard is 1 XOR 1 = 0. Alice will lose.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,1]
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> true
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>0 <= nums[i] < 2<sup>16</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        nums.len() % 2 == 0 || nums.iter().fold(0, |x, y| x ^ y) == 0
    }
}
```
