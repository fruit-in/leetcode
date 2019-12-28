# 836. 矩形重叠
矩形以列表 ```[x1, y1, x2, y2]``` 的形式表示，其中 ```(x1, y1)``` 为左下角的坐标，```(x2, y2)``` 是右上角的坐标。

如果相交的面积为正，则称两矩形重叠。需要明确的是，只在角或边接触的两个矩形不构成重叠。

给出两个矩形，判断它们是否重叠并返回结果。

#### 示例 1:
<pre>
<strong>输入:</strong> rec1 = [0,0,2,2], rec2 = [1,1,3,3]
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rec1 = [0,0,1,1], rec2 = [1,0,2,1]
<strong>输出:</strong> false
</pre>

#### 说明:
1. 两个矩形 ```rec1``` 和 ```rec2``` 都以含有四个整数的列表的形式给出。
2. 矩形中的所有坐标都处于 ```-10^9``` 和 ```10^9``` 之间。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        rec1[0].max(rec2[0]) < rec1[2].min(rec2[2]) && rec1[1].max(rec2[1]) < rec1[3].min(rec2[3])
    }
}
```
