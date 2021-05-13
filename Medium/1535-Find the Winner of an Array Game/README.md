# 1535. Find the Winner of an Array Game
Given an integer array `arr` of **distinct** integers and an integer `k`.

A game will be played between the first two elements of the array (i.e. `arr[0]` and `arr[1]`). In each round of the game, we compare `arr[0]` with `arr[1]`, the larger integer wins and remains at position `0` and the smaller integer moves to the end of the array. The game ends when an integer wins `k` consecutive rounds.

Return *the integer which will win the game*.

It is **guaranteed** that there will be a winner of the game.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,1,3,5,4,6,7], k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> Let's see the rounds of the game:
Round |       arr       | winner | win_count
  1   | [2,1,3,5,4,6,7] | 2      | 1
  2   | [2,3,5,4,6,7,1] | 3      | 1
  3   | [3,5,4,6,7,1,2] | 5      | 1
  4   | [5,4,6,7,1,2,3] | 5      | 2
So we can see that 4 rounds will be played and 5 is the winner because it wins 2 consecutive games.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [3,2,1], k = 10
<strong>Output:</strong> 3
<strong>Explanation:</strong> 3 will win the first 10 rounds consecutively.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,9,8,2,3,7,6,4,5], k = 7
<strong>Output:</strong> 9
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [1,11,22,33,44,55,66,77,88,99], k = 1000000000
<strong>Output:</strong> 99
</pre>

#### Constraints:
* `2 <= arr.length <= 10^5`
* `1 <= arr[i] <= 10^6`
* `arr` contains **distinct** integers.
* `1 <= k <= 10^9`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def get_winner(arr, k)
  winner = arr[0]
  wins = 0

  arr[1..].each do |x|
    if x > winner
      winner = x
      wins = 0
    end
    wins += 1
    break if wins == k
  end

  winner
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let mut winner = arr[0];
        let mut wins = 0;

        for &x in &arr[1..] {
            if x > winner {
                winner = x;
                wins = 0;
            }
            wins += 1;
            if wins == k {
                break;
            }
        }

        winner
    }
}
```
