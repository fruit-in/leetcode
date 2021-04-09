# 1725. 可以形成最大正方形的矩形数目
给你一个数组 `rectangles` ，其中 <code>rectangles[i] = [l<sub>i</sub>, w<sub>i</sub>]</code> 表示第 `i` 个矩形的长度为 <code>l<sub>i</sub></code> 、宽度为 <code>w<sub>i</sub></code> 。

如果存在 `k` 同时满足 <code>k <= l<sub>i</sub></code> 和 <code>k <= w<sub>i</sub></code> ，就可以将第 `i` 个矩形切成边长为 `k` 的正方形。例如，矩形 `[4,6]` 可以切成边长最大为 `4` 的正方形。

设 `maxLen` 为可以从矩形数组 `rectangles` 切分得到的 **最大正方形** 的边长。

返回可以切出边长为 `maxLen` 的正方形的矩形 **数目** 。

#### 示例 1:
<pre>
<strong>输入:</strong> rectangles = [[5,8],[3,9],[5,12],[16,5]]
<strong>输出:</strong> 3
<strong>解释:</strong> 能从每个矩形中切出的最大正方形边长分别是 [5,3,5,5] 。
最大正方形的边长为 5 ，可以由 3 个矩形切分得到。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rectangles = [[2,3],[3,7],[4,3],[3,7]]
<strong>输出:</strong> 3
</pre>

#### 提示:
* `1 <= rectangles.length <= 1000`
* `rectangles[i].length == 2`
* <code>1 <= l<sub>i</sub>, w<sub>i</sub> <= 10<sup>9</sup></code>
* <code>l<sub>i</sub> != w<sub>i</sub></code>

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} rectangles
# @return {Integer}
def count_good_rectangles(rectangles)
  max_len = 0
  ret = 0

  rectangles.each do |rec|
    if rec.min == max_len
      ret += 1
    elsif rec.min > max_len
      max_len = rec.min
      ret = 1
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_len = 0;
        let mut ret = 0;

        for rec in rectangles {
            if rec[0].min(rec[1]) == max_len {
                ret += 1;
            } else if rec[0].min(rec[1]) > max_len {
                max_len = rec[0].min(rec[1]);
                ret = 1;
            }
        }

        ret
    }
}
```
