# 1222. 可以攻击国王的皇后
在一个 **8x8** 的棋盘上，放置着若干「黑皇后」和一个「白国王」。

「黑皇后」在棋盘上的位置分布用整数坐标数组 ```queens``` 表示，「白国王」的坐标用数组 ```king``` 表示。

「黑皇后」的行棋规定是：横、直、斜都可以走，步数不受限制，但是，不能越子行棋。

请你返回可以直接攻击到「白国王」的所有「黑皇后」的坐标（任意顺序）。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/10/13/untitled-diagram.jpg)
<pre>
<strong>输入:</strong> queens = [[0,1],[1,0],[4,0],[0,4],[3,3],[2,4]], king = [0,0]
<strong>输出:</strong> [[0,1],[1,0],[3,3]]
<strong>解释:</strong>
[0,1] 的皇后可以攻击到国王，因为他们在同一行上。
[1,0] 的皇后可以攻击到国王，因为他们在同一列上。
[3,3] 的皇后可以攻击到国王，因为他们在同一条对角线上。
[0,4] 的皇后无法攻击到国王，因为她被位于 [0,1] 的皇后挡住了。
[4,0] 的皇后无法攻击到国王，因为她被位于 [1,0] 的皇后挡住了。
[2,4] 的皇后无法攻击到国王，因为她和国王不在同一行/列/对角线上。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/10/13/untitled-diagram-1.jpg)
<pre>
<strong>输入:</strong> queens = [[0,0],[1,1],[2,2],[3,4],[3,5],[4,4],[4,5]], king = [3,3]
<strong>输出:</strong> [[2,2],[3,4],[4,4]]
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/10/13/untitled-diagram-2.jpg)
<pre>
<strong>输入:</strong> queens = [[5,6],[7,7],[2,1],[0,7],[1,6],[5,1],[3,7],[0,3],[4,0],[1,2],[6,3],[5,0],[0,4],[2,2],[1,1],[6,4],[5,4],[0,0],[2,6],[4,5],[5,2],[1,4],[7,5],[2,3],[0,5],[4,2],[1,0],[2,7],[0,1],[4,6],[6,1],[0,6],[4,3],[1,7]], king = [3,4]
<strong>输出:</strong> [[2,3],[1,4],[1,6],[3,7],[4,3],[5,4],[4,5]]
</pre>

#### 提示:
* ```1 <= queens.length <= 63```
* ```queens[0].length == 2```
* ```0 <= queens[i][j] < 8```
* ```king.length == 2```
* ```0 <= king[0], king[1] < 8```
* 一个棋盘格上最多只能放置一枚棋子。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        let dir = vec![(-1, 0, king[0]),
                       (0, -1, king[1]),
                       (1, 0, 7 - king[0]),
                       (0, 1, 7 - king[1]),
                       (-1, 1, king[0].min(7 - king[1])),
                       (1, -1, king[1].min(7 - king[0])),
                       (-1, -1, king[0].min(king[1])),
                       (1, 1, 7 - king[0].max(king[1]))];

        for (a, b, c) in dir {
            for i in 1..=c {
                if queens.contains(&vec![king[0] + a * i, king[1] + b * i]) {
                    ret.push(vec![king[0] + a * i, king[1] + b * i]);
                    break;
                }
            }
        }

        ret
    }
}
```
