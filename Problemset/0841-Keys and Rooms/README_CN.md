# 841. 钥匙和房间
有 ```N``` 个房间，开始时你位于 ```0``` 号房间。每个房间有不同的号码：```0，1，2，...，N-1```，并且房间里可能有一些钥匙能使你进入下一个房间。

在形式上，对于每个房间 ```i``` 都有一个钥匙列表 ```rooms[i]```，每个钥匙 ```rooms[i][j]``` 由 ```[0,1，...，N-1]``` 中的一个整数表示，其中 ```N = rooms.length```。 钥匙 ```rooms[i][j] = v``` 可以打开编号为 ```v``` 的房间。

最初，除 ```0``` 号房间外的其余所有房间都被锁住。

你可以自由地在房间之间来回走动。

如果能进入每个房间返回 ```true```，否则返回 ```false```。

#### 示例 1:
<pre>
<strong>输入:</strong> [[1],[2],[3],[]]
<strong>输出:</strong> true
<strong>解释:</strong>
我们从 0 号房间开始，拿到钥匙 1。
之后我们去 1 号房间，拿到钥匙 2。
然后我们去 2 号房间，拿到钥匙 3。
最后我们去了 3 号房间。
由于我们能够进入每个房间，我们返回 true。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[1,3],[3,0,1],[2],[0]]
<strong>输出:</strong> false
<strong>解释:</strong> 我们不能进入 2 号房间。
</pre>

#### 提示:
1. ```1 <= rooms.length <= 1000```
2. ```0 <= rooms[i].length <= 1000```
3. 所有房间中的钥匙数量总计不超过 ```3000```。

## 题解 (Rust)

### 1. 深度优先搜索
```Rust
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut opened = vec![false; rooms.len()];
        let mut keys = vec![0];

        while let Some(curr) = keys.pop() {
            if !opened[curr] {
                opened[curr] = true;
                for &key in &rooms[curr] {
                    if !opened[key as usize] {
                        keys.push(key as usize);
                    }
                }
            }
        }

        opened.iter().all(|&room| room)
    }
}
```
