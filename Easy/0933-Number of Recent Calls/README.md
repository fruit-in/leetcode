# 933. Number of Recent Calls
Write a class ```RecentCounter``` to count recent requests.

It has only one method: ```ping(int t)```, where t represents some time in milliseconds.

Return the number of ```ping```s that have been made from 3000 milliseconds ago until now.

Any ping with time in ```[t - 3000, t]``` will count, including the current ping.

It is guaranteed that every call to ```ping``` uses a strictly larger value of ```t``` than before.

#### Example 1:
<pre>
<strong>Input:</strong> inputs = ["RecentCounter","ping","ping","ping","ping"], inputs = [[],[1],[100],[3001],[3002]]
<strong>Output:</strong> [null,1,2,3,3]
</pre>

#### Note:
1. Each test case will have at most ```10000``` calls to ```ping```.
2. Each test case will call ```ping``` with strictly increasing values of ```t```.
3. Each call to ping will have ```1 <= t <= 10^9```.

## Solutions (Rust)

### 1. Queue
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
