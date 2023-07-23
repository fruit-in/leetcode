# 365. Water and Jug Problem
You are given two jugs with capacities *x* and *y* litres. There is an infinite amount of water supply available. You need to determine whether it is possible to measure exactly *z* litres using these two jugs.

If *z* liters of water is measurable, you must have *z* liters of water contained within **one or both buckets** by the end.

Operations allowed:
* Fill any of the jugs completely with water.
* Empty any of the jugs.
* Pour water from one jug into another till the other jug is completely full or the first jug itself is empty.

**Example 1:** (From the famous [*"Die Hard"* example](https://www.youtube.com/watch?v=BVtQNK_ZUJg))
<pre>
<strong>Input:</strong> x = 3, y = 5, z = 4
<strong>Output:</strong> True
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> x = 2, y = 6, z = 5
<strong>Output:</strong> False
</pre>

#### Constraints:
* `0 <= x <= 10^6`
* `0 <= y <= 10^6`
* `0 <= z <= 10^6`

## Solutions (Ruby)

### 1. Mathematical
```Ruby
# @param {Integer} x
# @param {Integer} y
# @param {Integer} z
# @return {Boolean}
def can_measure_water(x, y, z)
  x + y >= z && (x + y == 0 || z % x.gcd(y) == 0)
end
```
