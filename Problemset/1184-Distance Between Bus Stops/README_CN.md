# 1184. 公交站间的距离
环形公交路线上有 ```n``` 个站，按次序从 ```0``` 到 ```n - 1``` 进行编号。我们已知每一对相邻公交站之间的距离，```distance[i]``` 表示编号为 ```i``` 的车站和编号为 ```(i + 1) % n``` 的车站之间的距离。

环线上的公交车都可以按顺时针和逆时针的方向行驶。

返回乘客从出发点 ```start``` 到目的地 ```destination``` 之间的最短距离。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/08/untitled-diagram-1.jpg)
<pre>
<strong>输入:</strong> distance = [1,2,3,4], start = 0, destination = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 公交站 0 和 1 之间的距离是 1 或 9，最小值是 1。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/08/untitled-diagram-1-1.jpg)
<pre>
<strong>输入:</strong> distance = [1,2,3,4], start = 0, destination = 2
<strong>输出:</strong> 3
<strong>解释:</strong> 公交站 0 和 2 之间的距离是 3 或 7，最小值是 3。
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/09/08/untitled-diagram-1-2.jpg)
<pre>
<strong>输入:</strong> distance = [1,2,3,4], start = 0, destination = 3
<strong>输出:</strong> 4
<strong>解释:</strong> 公交站 0 和 3 之间的距离是 6 或 4，最小值是 4。
</pre>

#### 提示:
* ```1 <= n <= 10^4```
* ```distance.length == n```
* ```0 <= start, destination < n```
* ```0 <= distance[i] <= 10^4```

## 题解 (Rust)

### 1. 比较顺时针和逆时针距离
```Rust
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut clockwise = 0;
        let mut curr = start;
        while curr != destination {
            clockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }

        let mut counterclockwise = 0;
        curr = start;
        while curr != destination {
            curr -= 1;
            curr = (curr + n) % n;
            counterclockwise += distance[curr as usize];
        }
        clockwise.min(counterclockwise)
    }
}
```

### 2. 总距离减去顺时针距离
```Rust
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut clockwise = 0;
        let mut curr = start;
        while curr != destination {
            clockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }

        let total_distance: i32 = distance.iter().sum();
        clockwise.min(total_distance - clockwise)
    }
}
```

### 3. 交换起点和终点
```Rust
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let n = distance.len() as i32;
        let mut clockwise = 0;
        let mut curr = start;
        while curr != destination {
            clockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }

        let mut counterclockwise = 0;
        curr = destination;
        while curr != start {
            counterclockwise += distance[curr as usize];
            curr += 1;
            curr %= n;
        }
        clockwise.min(counterclockwise)
    }
}
```
