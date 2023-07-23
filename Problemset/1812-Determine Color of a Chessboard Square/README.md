# 1812. Determine Color of a Chessboard Square
You are given `coordinates`, a string that represents the coordinates of a square of the chessboard. Below is a chessboard for your reference.

![](https://assets.leetcode.com/uploads/2021/02/19/screenshot-2021-02-20-at-22159-pm.png)

Return `true` *if the square is white, and* `false` *if the square is black*.

The coordinate will always represent a valid chessboard square. The coordinate will always have the letter first, and the number second.

#### Example 1:
<pre>
<strong>Input:</strong> coordinates = "a1"
<strong>Output:</strong> false
<strong>Explanation:</strong> From the chessboard above, the square with coordinates "a1" is black, so return false.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> coordinates = "h3"
<strong>Output:</strong> true
<strong>Explanation:</strong> From the chessboard above, the square with coordinates "h3" is white, so return true.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> coordinates = "c7"
<strong>Output:</strong> false
</pre>

#### Constraints:
* `coordinates.length == 2`
* `'a' <= coordinates[0] <= 'h'`
* `'1' <= coordinates[1] <= '8'`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def squareIsWhite(self, coordinates: str) -> bool:
        return ord(coordinates[0]) % 2 != ord(coordinates[1]) % 2
```

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String} coordinates
# @return {Boolean}
def square_is_white(coordinates)
  coordinates[0].ord % 2 != coordinates[1].ord % 2
end
```
