# 789. Escape The Ghosts
You are playing a simplified Pacman game. You start at the point `(0, 0)`, and your destination is `(target[0], target[1])`. There are several ghosts on the map, the i-th ghost starts at `(ghosts[i][0], ghosts[i][1])`.

Each turn, you and all ghosts simultaneously *may* move in one of 4 cardinal directions: north, east, west, or south, going from the previous point to a new point 1 unit of distance away.

You escape if and only if you can reach the target before any ghost reaches you (for any given moves the ghosts may take.)  If you reach any square (including the target) at the same time as a ghost, it doesn't count as an escape.

Return True if and only if it is possible to escape.

#### Example 1:
<pre>
<strong>Input:</strong>
ghosts = [[1, 0], [0, 3]]
target = [0, 1]
<strong>Output:</strong> true
<strong>Explanation:</strong>
You can directly reach the destination (0, 1) at time 1, while the ghosts located at (1, 0) or (0, 3) have no way to catch up with you.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
ghosts = [[1, 0]]
target = [2, 0]
<strong>Output:</strong> false
<strong>Explanation:</strong>
You need to reach the destination (2, 0), but the ghost at (1, 0) lies between you and the destination.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong>
ghosts = [[2, 0]]
target = [1, 0]
<strong>Output:</strong> false
<strong>Explanation:</strong>
The ghost can reach the target at the same time as you.
</pre>

#### Note:
* All points have coordinates with absolute value <= `10000`.
* The number of ghosts will not exceed `100`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[][]} ghosts
# @param {Integer[]} target
# @return {Boolean}
def escape_ghosts(ghosts, target)
    min_distance = 20001

    ghosts.each do |ghost|
        distance = (ghost[0] - target[0]).abs + (ghost[1] - target[1]).abs
        min_distance = distance if distance < min_distance
    end

    return min_distance > target[0].abs + target[1].abs
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        ghosts
            .iter()
            .map(|v| (v[0] - target[0]).abs() + (v[1] - target[1]).abs())
            .min()
            .unwrap_or(20001)
            > (target[0].abs() + target[1].abs())
    }
}
```
