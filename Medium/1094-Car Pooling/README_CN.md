# 1094. 拼车
假设你是一位顺风车司机，车上最初有 ```capacity``` 个空座位可以用来载客。由于道路的限制，车 **只能** 向一个方向行驶（也就是说，**不允许掉头或改变方向**，你可以将其想象为一个向量）。

这儿有一份行程计划表 ```trips[][]```，其中 ```trips[i] = [num_passengers, start_location, end_location]``` 包含了你的第 ```i``` 次行程信息：
* 必须接送的乘客数量；
* 乘客的上车地点；
* 以及乘客的下车地点。

这些给出的地点位置是从你的 **初始** 出发位置向前行驶到这些地点所需的距离（它们一定在你的行驶方向上）。

请你根据给出的行程计划表和车子的座位数，来判断你的车是否可以顺利完成接送所用乘客的任务（当且仅当你可以在所有给定的行程中接送所有乘客时，返回 ```true```，否则请返回 ```false```）。

#### 示例 1:
<pre>
<strong>输入:</strong> trips = [[2,1,5],[3,3,7]], capacity = 4
<strong>输出:</strong> false
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> trips = [[2,1,5],[3,3,7]], capacity = 5
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> trips = [[2,1,5],[3,5,7]], capacity = 3
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> trips = [[3,2,7],[3,7,9],[8,3,9]], capacity = 11
<strong>输出:</strong> true
</pre>

#### 提示:
1. 你可以假设乘客会自觉遵守 “**先下后上**” 的良好素质
2. ```trips.length <= 1000```
3. ```trips[i].length == 3```
4. ```1 <= trips[i][0] <= 100```
5. ```0 <= trips[i][1] < trips[i][2] <= 1000```
6. ```1 <= capacity <= 100000```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut pick = [0; 1001];

        for trip in trips {
            pick[trip[2] as usize] -= trip[0];
            pick[trip[1] as usize] += trip[0];
        }

        for i in 1..1001 {
            pick[i] += pick[i - 1];
            if pick[i] > capacity {
                return false;
            }
        }

        pick[0] <= capacity
    }
}
```
