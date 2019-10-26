# 1033. Moving Stones Until Consecutive
Three stones are on a number line at positions ```a```, ```b```, and ```c```.

Each turn, you pick up a stone at an endpoint (ie., either the lowest or highest position stone), and move it to an unoccupied position between those endpoints.  Formally, let's say the stones are currently at positions ```x, y, z``` with ```x < y < z```.  You pick up the stone at either position ```x``` or position ```z```, and move that stone to an integer position ```k```, with ```x < k < z``` and ```k != y```.

The game ends when you cannot make any more moves, ie. the stones are in consecutive positions.

When the game ends, what is the minimum and maximum number of moves that you could have made?  Return the answer as an length 2 array: ```answer = [minimum_moves, maximum_moves]```

#### Example 1:
<pre>
<strong>Input:</strong> a = 1, b = 2, c = 5
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> Move the stone from 5 to 3, or move the stone from 5 to 4 to 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = 4, b = 3, c = 2
<strong>Output:</strong> [0,0]
<strong>Explanation:</strong> We cannot make any moves.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> a = 3, b = 5, c = 1
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> Move the stone from 1 to 4; or move the stone from 1 to 2 to 4.
</pre>

#### Note:
1. ```1 <= a <= 100```
2. ```1 <= b <= 100```
3. ```1 <= c <= 100```
4. ```a != b, b != c, c != a```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut xyz = vec![a, b, c];
        xyz.sort_unstable();
        let x = xyz[0];
        let y = xyz[1];
        let z = xyz[2];

        if z - x == 2 {
            vec![0, 0]
        } else if z - y > 2 && y - x > 2 {
            vec![2, z - x - 2]
        } else {
            vec![1, z - x - 2]
        }
    }
}
```
