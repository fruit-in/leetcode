# 452. 用最少数量的箭引爆气球
在二维空间中有许多球形的气球。对于每个气球，提供的输入是水平方向上，气球直径的开始和结束坐标。由于它是水平的，所以纵坐标并不重要，因此只要知道开始和结束的横坐标就足够了。开始坐标总是小于结束坐标。

一支弓箭可以沿着 x 轴从不同点完全垂直地射出。在坐标 x 处射出一支箭，若有一个气球的直径的开始和结束坐标为 <code>x<sub>start</sub></code>，<code>x<sub>end</sub></code>， 且满足  <code>x<sub>start</sub> ≤ x ≤ x<sub>end</sub></code>，则该气球会被引爆。可以射出的弓箭的数量没有限制。 弓箭一旦被射出之后，可以无限地前进。我们想找到使得所有气球全部被引爆，所需的弓箭的最小数量。

给你一个数组 `points` ，其中 <code>points [i] = [x<sub>start</sub>,x<sub>end</sub>]</code> ，返回引爆所有气球所必须射出的最小弓箭数。

#### 示例 1:
<pre>
<strong>输入:</strong> points = [[10,16],[2,8],[1,6],[7,12]]
<strong>输出:</strong> 2
<strong>解释:</strong> 对于该样例，x = 6 可以射爆 [2,8],[1,6] 两个气球，以及 x = 11 射爆另外两个气球
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> points = [[1,2],[3,4],[5,6],[7,8]]
<strong>输出:</strong> 4
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> points = [[1,2],[2,3],[3,4],[4,5]]
<strong>输出:</strong> 2
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> points = [[1,2]]
<strong>输出:</strong> 1
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> points = [[2,3],[2,3]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= points.length <= 10<sup>4</sup></code>
* `points[i].length == 2`
* <code>-2<sup>31</sup> <= x<sub>start</sub> < x<sub>end</sub> <= 2<sup>31</sup> - 1</code>

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[][]} points
# @return {Integer}
def find_min_arrow_shots(points)
  points.sort_by! { |p| p[1] }
  x = points[0][1]
  ret = 1

  (1...points.size).each do |i|
    if points[i][0] > x
      x = points[i][1]
      ret += 1
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|p| p[1]);
        let mut x = points[0][1];
        let mut ret = 1;

        for i in 1..points.len() {
            if points[i][0] > x {
                x = points[i][1];
                ret += 1;
            }
        }

        ret
    }
}
```
