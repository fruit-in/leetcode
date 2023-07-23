# 2101. Detonate the Maximum Bombs
You are given a list of bombs. The **range** of a bomb is defined as the area where its effect can be felt. This area is in the shape of a **circle** with the center as the location of the bomb.

The bombs are represented by a **0-indexed** 2D integer array `bombs` where <code>bombs[i] = [x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub>]</code>. <code>x<sub>i</sub></code> and <code>yi</code> denote the X-coordinate and Y-coordinate of the location of the <code>i<sup>th</sup></code> bomb, whereas <code>r<sub>i</sub></code> denotes the **radius** of its range.

You may choose to detonate a **single** bomb. When a bomb is detonated, it will detonate **all bombs** that lie in its range. These bombs will further detonate the bombs that lie in their ranges.

Given the list of `bombs`, return *the **maximum** number of bombs that can be detonated if you are allowed to detonate **only one** bomb*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/11/06/desmos-eg-3.png)
<pre>
<strong>Input:</strong> bombs = [[2,1,3],[6,1,4]]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The above figure shows the positions and ranges of the 2 bombs.
If we detonate the left bomb, the right bomb will not be affected.
But if we detonate the right bomb, both bombs will be detonated.
So the maximum bombs that can be detonated is max(1, 2) = 2.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/11/06/desmos-eg-2.png)
<pre>
<strong>Input:</strong> bombs = [[1,1,5],[10,10,5]]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Detonating either bomb will not detonate the other bomb, so the maximum number of bombs that can be detonated is 1.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/11/07/desmos-eg1.png)
<pre>
<strong>Input:</strong> bombs = [[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]]
<strong>Output:</strong> 5
<strong>Explanation:</strong>
The best bomb to detonate is bomb 0 because:
- Bomb 0 detonates bombs 1 and 2. The red circle denotes the range of bomb 0.
- Bomb 2 detonates bomb 3. The blue circle denotes the range of bomb 2.
- Bomb 3 detonates bomb 4. The green circle denotes the range of bomb 3.
Thus all 5 bombs are detonated.
</pre>

#### Constraints:
* `1 <= bombs.length <= 100`
* `bombs[i].length == 3`
* <code>1 <= x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub> <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut edges = vec![vec![]; bombs.len()];
        let mut visited = HashSet::new();
        let mut stack = vec![];
        let mut ret = 0;

        for i in 0..bombs.len() {
            for j in 0..bombs.len() {
                let x2 = ((bombs[i][0] - bombs[j][0]) as i64).pow(2);
                let y2 = ((bombs[i][1] - bombs[j][1]) as i64).pow(2);
                let r2 = (bombs[i][2] as i64).pow(2);

                if x2 + y2 <= r2 {
                    edges[i].push(j);
                }
            }
        }

        for i in 0..bombs.len() {
            visited.clear();
            visited.insert(i);
            stack.clear();
            stack.push(i);

            while let Some(j) = stack.pop() {
                for &k in &edges[j] {
                    if !visited.contains(&k) {
                        visited.insert(k);
                        stack.push(k);
                    }
                }
            }

            ret = ret.max(visited.len());
        }

        ret as i32
    }
}
```
