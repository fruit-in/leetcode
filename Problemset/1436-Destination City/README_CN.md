# 1436. 旅行终点站
给你一份旅游线路图，该线路图中的旅行线路用数组 `paths` 表示，其中 <code>paths[i] = [cityA<sub>i</sub>, cityB<sub>i</sub>]</code> 表示该线路将会从 <code>cityA<sub>i</sub></code> 直接前往 <code>cityB<sub>i</sub></code> 。请你找出这次旅行的终点站，即没有任何可以通往其他城市的线路的城市。

题目数据保证线路图会形成一条不存在循环的线路，因此只会有一个旅行终点站。

#### 示例 1:
<pre>
<strong>输入:</strong> paths = [["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]
<strong>输出:</strong> "Sao Paulo"
<strong>解释:</strong> 从 "London" 出发，最后抵达终点站 "Sao Paulo" 。本次旅行的路线是 "London" -> "New York" -> "Lima" -> "Sao Paulo" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> paths = [["B","C"],["D","B"],["C","A"]]
<strong>输出:</strong> "A"
<strong>解释:</strong> 所有可能的线路是：
"D" -> "B" -> "C" -> "A". 
"B" -> "C" -> "A". 
"C" -> "A". 
"A". 
显然，旅行终点站是 "A" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> paths = [["A","Z"]]
<strong>输出:</strong> "Z"
</pre>

#### 提示:
* `1 <= paths.length <= 100`
* `paths[i].length == 2`
* <code>1 <= cityA<sub>i</sub>.length, cityB<sub>i</sub>.length <= 10</code>
* <code>cityA<sub>i</sub> != cityB<sub>i</sub></code>
* 所有字符串均由大小写英文字母和空格字符组成。

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let city_a = paths.iter().map(|path| &path[0]).collect::<HashSet<_>>();

        paths
            .iter()
            .find(|path| !city_a.contains(&path[1]))
            .unwrap()[1]
            .clone()
    }
}
```
