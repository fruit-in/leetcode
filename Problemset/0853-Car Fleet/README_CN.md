# 853. 车队
`N` 辆车沿着一条车道驶向位于 `target` 英里之外的共同目的地。

每辆车 `i` 以恒定的速度 `speed[i]` （英里/小时），从初始位置 `position[i]` （英里） 沿车道驶向目的地。

一辆车永远不会超过前面的另一辆车，但它可以追上去，并与前车以相同的速度紧接着行驶。

此时，我们会忽略这两辆车之间的距离，也就是说，它们被假定处于相同的位置。

*车队* 是一些由行驶在相同位置、具有相同速度的车组成的非空集合。注意，一辆车也可以是一个车队。

即便一辆车在目的地才赶上了一个车队，它们仍然会被视作是同一个车队。

会有多少车队到达目的地?

#### 示例:
<pre>
<strong>输入:</strong> target = 12, position = [10,8,0,5,3], speed = [2,4,1,1,3]
<strong>输出:</strong> 3
<strong>解释:</strong>
从 10 和 8 开始的车会组成一个车队，它们在 12 处相遇。
从 0 处开始的车无法追上其它车，所以它自己就是一个车队。
从 5 和 3 开始的车会组成一个车队，它们在 6 处相遇。
请注意，在到达目的地之前没有其它车会遇到这些车队，所以答案是 3。
</pre>

#### 提示:
1. `0 <= N <= 10 ^ 4`
2. `0 < target <= 10 ^ 6`
3. `0 < speed[i] <= 10 ^ 6`
4. `0 <= position[i] < target`
5. 所有车的初始位置各不相同。

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        if position.len() == 0 {
            return 0;
        }

        let mut cars = position.iter().zip(speed.iter()).collect::<Vec<_>>();
        cars.sort_unstable();
        let (&p, &s) = cars.pop().unwrap();
        let mut time = (target - p) as f64 / s as f64;
        let mut ret = 1;

        while let Some((&p, &s)) = cars.pop() {
            let t = (target - p) as f64 / s as f64;
            if time < t {
                ret += 1;
                time = t;
            }
        }

        ret
    }
}
```
