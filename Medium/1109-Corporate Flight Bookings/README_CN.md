# 1109. 航班预订统计
这里有 `n` 个航班，它们分别从 `1` 到 `n` 进行编号。

我们这儿有一份航班预订表，表中第 `i` 条预订记录 `bookings[i] = [i, j, k]` 意味着我们在从 `i` 到 `j` 的每个航班上预订了 `k` 个座位。

请你返回一个长度为 `n` 的数组 `answer`，按航班编号顺序返回每个航班上预订的座位数。

#### 示例:
<pre>
<b>输入:</b> bookings = [[1,2,10],[2,3,20],[2,5,25]], n = 5
<b>输出:</b> [10,55,45,25,25]
</pre>

#### 提示:
* `1 <= bookings.length <= 20000`
* `1 <= bookings[i][0] <= bookings[i][1] <= n <= 20000`
* `1 <= bookings[i][2] <= 10000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ret = vec![0; n as usize];

        for booking in bookings {
            if booking[0] > 1 {
                ret[booking[0] as usize - 2] -= booking[2];
            }
            ret[booking[1] as usize - 1] += booking[2];
        }

        for i in (0..(n as usize - 1)).rev() {
            ret[i] += ret[i + 1];
        }

        ret
    }
}
```
