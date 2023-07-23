# 365. 水壶问题
有两个容量分别为 *x*升 和 *y*升 的水壶以及无限多的水。请判断能否通过使用这两个水壶，从而可以得到恰好 *z*升 的水？

如果可以，最后请用以上水壶中的一或两个来盛放取得的 *z*升 水。

你允许：
* 装满任意一个水壶
* 清空任意一个水壶
* 从一个水壶向另外一个水壶倒水，直到装满或者倒空

**示例 1:** (From the famous [*"Die Hard"* example](https://www.youtube.com/watch?v=BVtQNK_ZUJg))
<pre>
<strong>输入:</strong> x = 3, y = 5, z = 4
<strong>输出:</strong> True
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> x = 2, y = 6, z = 5
<strong>输出:</strong> False
</pre>

## 题解 (Ruby)

### 1. 数学
```Ruby
# @param {Integer} x
# @param {Integer} y
# @param {Integer} z
# @return {Boolean}
def can_measure_water(x, y, z)
  x + y >= z && (x + y == 0 || z % x.gcd(y) == 0)
end
```
