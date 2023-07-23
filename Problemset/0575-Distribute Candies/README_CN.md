# 575. 分糖果
给定一个**偶数**长度的数组，其中不同的数字代表着不同种类的糖果，每一个数字代表一个糖果。你需要把这些糖果**平均**分给一个弟弟和一个妹妹。返回妹妹可以获得的最大糖果的种类数。

#### 示例 1:
<pre>
<strong>输入:</strong> candies = [1,1,2,2,3,3]
<strong>输出:</strong> 3
<strong>解析:</strong> 一共有三种种类的糖果，每一种都有两个。
     最优分配方案：妹妹获得[1,2,3],弟弟也获得[1,2,3]。这样使妹妹获得糖果的种类数最多。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> candies = [1,1,2,3]
<strong>输出:</strong> 2
<strong>解析:</strong> 妹妹获得糖果[2,3],弟弟获得糖果[1,1]，妹妹有两种不同的糖果，弟弟只有一种。这样使得妹妹可以获得的糖果种类数最多。
</pre>

#### 注意:
1. 数组的长度为[2, 10,000]，并且确定为偶数。
2. 数组中数字的大小在范围[-100,000, 100,000]内。

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let sister: HashSet<_> = candies.iter().collect();
        sister.len().min(candies.len() / 2) as i32
    }
}
```

### 2. 排序
```Rust
impl Solution {
    pub fn distribute_candies(candies: Vec<i32>) -> i32 {
        let mut candies = candies;
        candies.sort_unstable();

        let mut sister = 1;

        for i in 1..candies.len() {
            if candies[i] != candies[i - 1] {
                sister += 1;
            }
        }

        sister.min(candies.len() / 2) as i32
    }
}
```
