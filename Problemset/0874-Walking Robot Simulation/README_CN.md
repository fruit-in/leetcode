# 874. 模拟行走机器人
机器人在一个无限大小的网格上行走，从点 (0, 0) 处开始出发，面向北方。该机器人可以接收以下三种类型的命令：
* ```-2```：向左转 90 度
* ```-1```：向右转 90 度
* ```1 <= x <= 9```：向前移动 ```x``` 个单位长度

在网格上有一些格子被视为障碍物。

第 ```i``` 个障碍物位于网格点  ```(obstacles[i][0], obstacles[i][1])```

如果机器人试图走到障碍物上方，那么它将停留在障碍物的前一个网格方块上，但仍然可以继续该路线的其余部分。

返回从原点到机器人的最大欧式距离的**平方**。

#### 示例 1:
<pre>
<strong>输入:</strong> commands = [4,-1,3], obstacles = []
<strong>输出:</strong> 25
<strong>解释:</strong> 机器人将会到达 (3, 4)
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> commands = [4,-1,4,-2,4], obstacles = [[2,4]]
<strong>输出:</strong> 65
<strong>解释:</strong> 机器人在左转走到 (1, 8) 之前将被困在 (1, 4) 处
</pre>

#### 提示:
1. ```0 <= commands.length <= 10000```
2. ```0 <= obstacles.length <= 10000```
3. ```-30000 <= obstacle[i][0] <= 30000```
4. ```-30000 <= obstacle[i][1] <= 30000```
5. 答案保证小于 ```2 ^ 31```

## 题解 (Rust)

### 1. 模拟
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
