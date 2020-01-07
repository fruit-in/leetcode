# 933. 最近的请求次数
写一个 ```RecentCounter``` 类来计算最近的请求。

它只有一个方法：```ping(int t)```，其中 ```t``` 代表以毫秒为单位的某个时间。

返回从 3000 毫秒前到现在的 ```ping``` 数。

任何处于 ```[t - 3000, t]``` 时间范围之内的 ```ping``` 都将会被计算在内，包括当前（指 ```t``` 时刻）的 ```ping```。

保证每次对 ```ping``` 的调用都使用比之前更大的 ```t``` 值。

#### 示例:
<pre>
<strong>输入:</strong> inputs = ["RecentCounter","ping","ping","ping","ping"], inputs = [[],[1],[100],[3001],[3002]]
<strong>输出:</strong> [null,1,2,3,3]
</pre>

#### 提示:
1. 每个测试用例最多调用 ```10000``` 次 ```ping```。
2. 每个测试用例会使用严格递增的 ```t``` 值来调用 ```ping```。
3. 每次调用 ```ping``` 都有 ```1 <= t <= 10^9```。

## 题解 (Rust)

### 1. 队列
```Rust
struct RecentCounter {
    time: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self {
            time: Vec::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.time.push(t);

        while t - self.time[0] > 3000 {
            self.time.remove(0);
        }

        self.time.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
```
