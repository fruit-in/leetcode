# 1854. 人口最多的年份
给你一个二维整数数组 `logs` ，其中每个 <code>logs[i] = [birth<sub>i</sub>, death<sub>i</sub>]</code> 表示第 `i` 个人的出生和死亡年份。

年份 `x` 的 **人口** 定义为这一年期间活着的人的数目。第 `i` 个人被计入年份 `x` 的人口需要满足：`x` 在闭区间 <code>[birth<sub>i</sub>, death<sub>i</sub> - 1]</code> 内。注意，人不应当计入他们死亡当年的人口中。

返回 **人口最多** 且 **最早** 的年份。

#### 示例 1:
<pre>
<strong>输入:</strong> logs = [[1993,1999],[2000,2010]]
<strong>输出:</strong> 1993
<strong>解释:</strong> 人口最多为 1 ，而 1993 是人口为 1 的最早年份。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> logs = [[1950,1961],[1960,1971],[1970,1981]]
<strong>输出:</strong> 1960
<strong>解释:</strong> 人口最多为 2 ，分别出现在 1960 和 1970 。
其中最早年份是 1960 。
</pre>

#### 提示:
* `1 <= logs.length <= 100`
* <code>1950 <= birth<sub>i</sub> < death<sub>i</sub> <= 2050</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        let mut ret = 1950;

        for year in 1950..=2050 {
            let population = logs
                .iter()
                .filter(|&log| (log[0]..log[1]).contains(&year))
                .count();

            if population > max {
                max = population;
                ret = year;
            }
        }

        ret
    }
}
```
