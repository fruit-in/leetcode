# 165. 比较版本号
比较两个版本号 *version1* 和 *version2*。
如果 *`version1 > version2`* 返回 `1`，如果 *`version1 < version2`* 返回 `-1`， 除此之外返回 `0`。

你可以假设版本字符串非空，并且只包含数字和 `.` 字符。

`.` 字符不代表小数点，而是用于分隔数字序列。

例如，`2.5` 不是“两个半”，也不是“差一半到三”，而是第二版中的第五个小版本。

你可以假设版本号的每一级的默认修订版号为 `0`。例如，版本号 `3.4` 的第一级（大版本）和第二级（小版本）修订号分别为 `3` 和 `4`。其第三级和第四级修订号均为 `0`。

#### 示例 1:
<pre>
<strong>输入:</strong> <em>version1</em> = "0.1", <em>version2</em> = "1.1"
<strong>输出:</strong> -1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> <em>version1</em> = "1.0.1", <em>version2</em> = "1"
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> <em>version1</em> = "7.5.2.4", <em>version2</em> = "7.5.3"
<strong>输出:</strong> -1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> <em>version1</em> = "1.01", <em>version2</em> = "1.001"
<strong>输出:</strong> 0
<strong>解释:</strong> 忽略前导零，“01” 和 “001” 表示相同的数字 “1”。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> <em>version1</em> = "1.0", <em>version2</em> = "1.0.0"
<strong>输出:</strong> 0
<strong>解释:</strong> <em>version1</em> 没有第三级修订号，这意味着它的第三级修订号默认为 “0”。
</pre>

#### 提示:
1. 版本字符串由以点 （`.`） 分隔的数字字符串组成。这个数字字符串**可能**有前导零。
2. 版本字符串不以点开始或结束，并且其中不会有两个连续的点。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1 = version1
            .split('.')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();
        let mut version2 = version2
            .split('.')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();

        while let Some(&0) = version1.last() {
            version1.pop();
        }
        while let Some(&0) = version2.last() {
            version2.pop();
        }

        match version1.cmp(&version2) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}
```
