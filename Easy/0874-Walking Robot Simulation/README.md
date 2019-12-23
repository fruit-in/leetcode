# 874. Walking Robot Simulation
A robot on an infinite grid starts at point (0, 0) and faces north.  The robot can receive one of three possible types of commands:
* ```-2```: turn left 90 degrees
* ```-1```: turn right 90 degrees
* ```1 <= x <= 9```: move forward ```x``` units

Some of the grid squares are obstacles.

The ```i```-th obstacle is at grid point ```(obstacles[i][0], obstacles[i][1])```

If the robot would try to move onto them, the robot stays on the previous grid square instead (but still continues following the rest of the route.)

Return the **square** of the maximum Euclidean distance that the robot will be from the origin.

#### Example 1:
<pre>
<strong>Input:</strong> commands = [4,-1,3], obstacles = []
<strong>Output:</strong> 25
<strong>Explanation:</strong> robot will go to (3, 4)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> commands = [4,-1,4,-2,4], obstacles = [[2,4]]
<strong>Output:</strong> 65
<strong>Explanation:</strong> robot will be stuck at (1, 4) before turning left and going to (1, 8)
</pre>

#### Note:
1. ```0 <= commands.length <= 10000```
2. ```0 <= obstacles.length <= 10000```
3. ```-30000 <= obstacle[i][0] <= 30000```
4. ```-30000 <= obstacle[i][1] <= 30000```
5. The answer is guaranteed to be less than ```2 ^ 31```.

## Solutions (Rust)

### 1. Simulation
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut ob_set = obstacles
            .iter()
            .map(|v| (v[0], v[1]))
            .collect::<HashSet<_>>();
        let mut x = 0;
        let mut y = 0;
        let mut dir = (0, 1);
        let mut max_dis = 0;

        for com in commands {
            if com == -1 {
                dir = (dir.1, -dir.0);
            } else if com == -2 {
                dir = (-dir.1, dir.0);
            } else {
                for _ in 0..com {
                    if !ob_set.contains(&(x + dir.0, y + dir.1)) {
                        x += dir.0;
                        y += dir.1;
                    }
                }
                max_dis = max_dis.max(x * x + y * y);
            }
        }

        max_dis
    }
}
```
