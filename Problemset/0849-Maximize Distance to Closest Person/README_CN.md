# 849. 到最近的人的最大距离
在一排座位（```seats```）中，```1``` 代表有人坐在座位上，```0``` 代表座位上是空的。

至少有一个空座位，且至少有一人坐在座位上。

亚历克斯希望坐在一个能够使他与离他最近的人之间的距离达到最大化的座位上。

返回他到离他最近的人的最大距离。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,0,0,0,1,0,1]
<strong>输出:</strong> 2
<strong>解释:</strong>
如果亚历克斯坐在第二个空位（seats[2]）上，他到离他最近的人的距离为 2 。
如果亚历克斯坐在其它任何一个空位上，他到离他最近的人的距离为 1 。
因此，他到离他最近的人的最大距离是 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,0,0,0]
<strong>输出:</strong> 3
<strong>解释:</strong>
如果亚历克斯坐在最后一个座位上，他离最近的人有 3 个座位远。
这是可能的最大距离，所以答案是 3 。
</pre>

#### 提示:
1. ```1 <= seats.length <= 20000```
2. ```seats``` 中只含有 0 和 1，至少有一个 ```0```，且至少有一个 ```1```。

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_distance = 0;

        for i in 0..seats.len() {
            if seats[i] == 0 {
                let mut left_distance = 0;
                for j in (0..=i).rev() {
                    if seats[j] == 1 {
                        break;
                    }
                    left_distance += 1;
                    if j == 0 {
                        left_distance += 20000;
                    }
                }

                let mut right_distance = 0;
                for j in i..seats.len() {
                    if seats[j] == 1 {
                        break;
                    }
                    right_distance += 1;
                    if j == seats.len() - 1{
                        right_distance += 20000;
                    }
                }

                max_distance = max_distance.max(left_distance.min(right_distance));
            }
        }

        max_distance
    }
}
```

### 2. 计数相邻的零
```Rust
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut max_empty = 0;

        let mut i = -1;
        for j in 0..(seats.len() + 1) {
            if j == seats.len() {
                max_empty = max_empty.max(2 * (j as i32 - i - 1));
            } else if seats[j] == 1 {
                if i == -1 {
                    max_empty = max_empty.max(2 * (j as i32 - i - 1));
                } else {
                    max_empty = max_empty.max(j as i32 - i - 1);
                }
                i = j as i32;
            }
        }

        (max_empty + 1) / 2
    }
}
```
