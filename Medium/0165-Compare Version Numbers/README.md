# 165. Compare Version Numbers
Compare two version numbers *version1* and *version2*.
If *`version1 > version2`* return `1;` if *`version1 < version2`* return `-1;`otherwise return `0`.

You may assume that the version strings are non-empty and contain only digits and the `.` character.

The `.` character does not represent a decimal point and is used to separate number sequences.

For instance, `2.5` is not "two and a half" or "half way to version three", it is the fifth second-level revision of the second first-level revision.

You may assume the default revision number for each level of a version number to be `0`. For example, version number `3.4` has a revision number of `3` and `4` for its first and second level revision number. Its third and fourth level revision number are both `0`.

#### Example 1:
<pre>
<strong>Input:</strong> <em>version1</em> = "0.1", <em>version2</em> = "1.1"
<strong>Output:</strong> -1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> <em>version1</em> = "1.0.1", <em>version2</em> = "1"
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> <em>version1</em> = "7.5.2.4", <em>version2</em> = "7.5.3"
<strong>Output:</strong> -1
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> <em>version1</em> = "1.01", <em>version2</em> = "1.001"
<strong>Output:</strong> 0
<strong>Explanation:</strong> Ignoring leading zeroes, both “01” and “001" represent the same number “1”
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> <em>version1</em> = "1.0", <em>version2</em> = "1.0.0"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The first version number does not have a third level revision number, which means its third level revision number is default to "0"
</pre>

#### Note:
1. Version strings are composed of numeric strings separated by dots `.` and this numeric strings **may** have leading zeroes. 
2. Version strings do not start or end with dots, and they will not be two consecutive dots.

## Solutions (Rust)

### 1. Solution
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
