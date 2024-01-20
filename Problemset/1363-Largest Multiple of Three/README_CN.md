# 1363. 形成三的最大倍数
给你一个整数数组 `digits`，你可以通过按 **任意顺序** 连接其中某些数字来形成 **3** 的倍数，请你返回所能得到的最大的 3 的倍数。

由于答案可能不在整数数据类型范围内，请以字符串形式返回答案。如果无法得到答案，请返回一个空字符串。返回的结果不应包含不必要的前导零。

#### 示例 1:
<pre>
<strong>输入:</strong> digits = [8,1,9]
<strong>Output:</strong> "981"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> digits = [8,6,7,1,0]
<strong>输出:</strong> "8760"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> digits = [1]
<strong>输出:</strong> ""
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> digits = [0,0,0,0,0,0]
<strong>输出:</strong> "0"
</pre>

#### 提示:
* <code>1 <= digits.length <= 10<sup>4</sup></code>
* `0 <= digits[i] <= 9`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut count = [0; 10];
        let mut sum_rem = 0;
        let mut min_rem = [vec![], vec![], vec![]];
        let mut ret = vec![];

        for d in digits {
            count[d as usize] += 1;
            sum_rem = (sum_rem + d) % 3;
        }

        for d in 1..9 {
            if min_rem[d % 3].len() < 2 && count[d] > 0 {
                min_rem[d % 3].push(d);
                if min_rem[d % 3].len() < 2 && count[d] > 1 {
                    min_rem[d % 3].push(d);
                }
            }
        }

        if sum_rem > 0 {
            if !min_rem[sum_rem as usize].is_empty() {
                count[min_rem[sum_rem as usize][0]] -= 1;
            } else {
                count[min_rem[3 - sum_rem as usize][0]] -= 1;
                count[min_rem[3 - sum_rem as usize][1]] -= 1;
            }
        }

        for d in (0..10).rev() {
            while count[d] > 0 {
                count[d] -= 1;
                ret.push(d as u8 + b'0');
            }
        }

        if *ret.get(0).unwrap_or(&1) == b'0' {
            return 0.to_string();
        }

        String::from_utf8(ret).unwrap()
    }
}
```
