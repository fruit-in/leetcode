# 1812. 判断国际象棋棋盘中一个格子的颜色
给你一个坐标 `coordinates` ，它是一个字符串，表示国际象棋棋盘中一个格子的坐标。下图是国际象棋棋盘示意图。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/03/chessboard.png)

如果所给格子的颜色是白色，请你返回 `true`，如果是黑色，请返回 `false` 。

给定坐标一定代表国际象棋棋盘上一个存在的格子。坐标第一个字符是字母，第二个字符是数字。

#### 示例 1:
<pre>
<strong>输入:</strong> coordinates = "a1"
<strong>输出:</strong> false
<strong>解释:</strong> 如上图棋盘所示，"a1" 坐标的格子是黑色的，所以返回 false 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> coordinates = "h3"
<strong>输出:</strong> true
<strong>解释:</strong> 如上图棋盘所示，"h3" 坐标的格子是白色的，所以返回 true 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> coordinates = "c7"
<strong>输出:</strong> false
</pre>

#### 提示:
* `coordinates.length == 2`
* `'a' <= coordinates[0] <= 'h'`
* `'1' <= coordinates[1] <= '8'`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def squareIsWhite(self, coordinates: str) -> bool:
        return ord(coordinates[0]) % 2 != ord(coordinates[1]) % 2
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} coordinates
# @return {Boolean}
def square_is_white(coordinates)
  coordinates[0].ord % 2 != coordinates[1].ord % 2
end
```
