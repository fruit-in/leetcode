# 735. 行星碰撞
给定一个整数数组 `asteroids`，表示在同一行的行星。

对于数组中的每一个元素，其绝对值表示行星的大小，正负表示行星的移动方向（正表示向右移动，负表示向左移动）。每一颗行星以相同的速度移动。

找出碰撞后剩下的所有行星。碰撞规则：两个行星相互碰撞，较小的行星会爆炸。如果两颗行星大小相同，则两颗行星都会爆炸。两颗移动方向相同的行星，永远不会发生碰撞。

#### 示例 1:
<pre>
<strong>输入:</strong> asteroids = [5,10,-5]
<strong>输出:</strong> [5,10]
<strong>解释:</strong> 10 和 -5 碰撞后只剩下 10 。 5 和 10 永远不会发生碰撞。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> asteroids = [8,-8]
<strong>输出:</strong> []
<strong>解释:</strong> 8 和 -8 碰撞后，两者都发生爆炸。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> asteroids = [10,2,-5]
<strong>输出:</strong> [10]
<strong>解释:</strong> 2 和 -5 发生碰撞后剩下 -5 。10 和 -5 发生碰撞后剩下 10 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> asteroids = [-2,-1,1,2]
<strong>输出:</strong> [-2,-1,1,2]
<strong>解释:</strong> -2 和 -1 向左移动，而 1 和 2 向右移动。 由于移动方向相同的行星不会发生碰撞，所以最终没有行星发生碰撞。
</pre>

#### 提示:
* <code>2 <= asteroids.length <= 10<sup>4</sup></code>
* `-1000 <= asteroids[i] <= 1000`
* `asteroids[i] != 0`

## 题解 (Ruby)

### 1. 栈
```Ruby
# @param {Integer[]} asteroids
# @return {Integer[]}
def asteroid_collision(asteroids)
  ret = []

  asteroids.each do |asteroid|
    if asteroid > 0
      ret.push(asteroid)
    else
      ret.pop while !ret.empty? && ret[-1] > 0 && ret[-1] < -asteroid
      if !ret.empty? && ret[-1] > 0 && ret[-1] == -asteroid
        ret.pop
      elsif ret.empty? || ret[-1] < 0
        ret.push(asteroid)
      end
    end
  end

  ret
end
```
