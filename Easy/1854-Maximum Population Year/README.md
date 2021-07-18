# 1854. Maximum Population Year
You are given a 2D integer array `logs` where each <code>logs[i] = [birth<sub>i</sub>, death<sub>i</sub>]</code> indicates the birth and death years of the <code>i<sup>th</sup></code> person.

The **population** of some year `x` is the number of people alive during that year. The <code>i<sup>th</sup></code> person is counted in year `x`'s population if `x` is in the **inclusive** range <code>[birth<sub>i</sub>, death<sub>i</sub> - 1]</code>. Note that the person is **not** counted in the year that they die.

Return *the **earliest** year with the **maximum population***.

#### Example 1:
<pre>
<strong>Input:</strong> logs = [[1993,1999],[2000,2010]]
<strong>Output:</strong> 1993
<strong>Explanation:</strong> The maximum population is 1, and 1993 is the earliest year with this population.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> logs = [[1950,1961],[1960,1971],[1970,1981]]
<strong>Output:</strong> 1960
<strong>Explanation:</strong> The maximum population is 2, and it had happened in years 1960 and 1970.
The earlier year between them is 1960.
</pre>

#### Constraints:
* `1 <= logs.length <= 100`
* <code>1950 <= birth<sub>i</sub> < death<sub>i</sub> <= 2050</code>

## Solutions (Rust)

### 1. Solution
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
