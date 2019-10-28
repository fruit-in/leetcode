# 1033. 移动石子直到连续
三枚石子放置在数轴上，位置分别为 ```a```，```b```，```c```。

每一回合，我们假设这三枚石子当前分别位于位置 ```x, y, z``` 且 ```x < y < z```。从位置 ```x``` 或者是位置 ```z``` 拿起一枚石子，并将该石子移动到某一整数位置 ```k``` 处，其中 ```x < k < z``` 且 ```k != y```。

当你无法进行任何移动时，即，这些石子的位置连续时，游戏结束。

要使游戏结束，你可以执行的最小和最大移动次数分别是多少？ 以长度为 2 的数组形式返回答案：```answer = [minimum_moves, maximum_moves]```

#### 示例 1:
<pre>
<strong>输入:</strong> a = 1, b = 2, c = 5
<strong>输出:</strong> [1,2]
<strong>解释:</strong> 将石子从 5 移动到 4 再移动到 3，或者我们可以直接将石子移动到 3。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> a = 4, b = 3, c = 2
<strong>输出:</strong> [0,0]
<strong>解释:</strong> 我们无法进行任何移动。
</pre>

#### 提示:
1. ```1 <= a <= 100```
2. ```1 <= b <= 100```
3. ```1 <= c <= 100```
4. ```a != b, b != c, c != a```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut xyz = vec![a, b, c];
        xyz.sort_unstable();
        let x = xyz[0];
        let y = xyz[1];
        let z = xyz[2];

        if z - x == 2 {
            vec![0, 0]
        } else if z - y > 2 && y - x > 2 {
            vec![2, z - x - 2]
        } else {
            vec![1, z - x - 2]
        }
    }
}
```
