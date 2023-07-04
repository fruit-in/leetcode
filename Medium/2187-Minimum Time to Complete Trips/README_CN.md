# 2187. 完成旅途的最少时间
给你一个数组 `time` ，其中 `time[i]` 表示第 `i` 辆公交车完成 **一趟旅途** 所需要花费的时间。

每辆公交车可以 **连续** 完成多趟旅途，也就是说，一辆公交车当前旅途完成后，可以 **立马开始** 下一趟旅途。每辆公交车 **独立** 运行，也就是说可以同时有多辆公交车在运行且互不影响。

给你一个整数 `totalTrips` ，表示所有公交车 **总共** 需要完成的旅途数目。请你返回完成 **至少** `totalTrips` 趟旅途需要花费的 **最少** 时间。

#### 示例 1:
<pre>
<strong>输入:</strong> time = [1,2,3], totalTrips = 5
<strong>输出:</strong> 3
<strong>解释:</strong>
- 时刻 t = 1 ，每辆公交车完成的旅途数分别为 [1,0,0] 。
  已完成的总旅途数为 1 + 0 + 0 = 1 。
- 时刻 t = 2 ，每辆公交车完成的旅途数分别为 [2,1,0] 。
  已完成的总旅途数为 2 + 1 + 0 = 3 。
- 时刻 t = 3 ，每辆公交车完成的旅途数分别为 [3,1,1] 。
  已完成的总旅途数为 3 + 1 + 1 = 5 。
所以总共完成至少 5 趟旅途的最少时间为 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> time = [2], totalTrips = 1
<strong>输出:</strong> 2
<strong>解释:</strong>
只有一辆公交车，它将在时刻 t = 2 完成第一趟旅途。
所以完成 1 趟旅途的最少时间为 2 。
</pre>

#### 提示:
* <code>1 <= time.length <= 10<sup>5</sup></code>
* <code>1 <= time[i], totalTrips <= 10<sup>7</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i64;
        let mut l = 1_i64;
        let mut r = total_trips * *time.iter().min().unwrap() as i64;

        while l < r {
            let m = (l + r) / 2;
            let trips = time.iter().map(|&t| m / t as i64).sum::<i64>();

            if trips < total_trips {
                l = m + 1;
            } else {
                r = m;
            }
        }

        r
    }
}
```
