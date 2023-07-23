# 13. 罗马数字转整数
罗马数字包含以下七种字符: ```I```， ```V```， ```X```， ```L```，```C```，```D``` 和 ```M```。

<pre>
<strong>字符</strong>          <strong>数值</strong>
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
</pre>



例如， 罗马数字 2 写做 ```II``` ，即为两个并列的 1。12 写做 ```XII``` ，即为 ```X``` + ```II``` 。 27 写做  ```XXVII```, 即为 ```XX``` + ```V``` + ```II``` 。

通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 ```IIII```，而是 ```IV```。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 ```IX```。这个特殊的规则只适用于以下六种情况：
* ```I``` 可以放在 ```V``` (5) 和 ```X``` (10) 的左边，来表示 4 和 9。
* ```X``` 可以放在 ```L``` (50) 和 ```C``` (100) 的左边，来表示 40 和 90。
* ```C``` 可以放在 ```D``` (500) 和 ```M``` (1000) 的左边，来表示 400 和 900。

给定一个罗马数字，将其转换成整数。输入确保在 1 到 3999 的范围内。

#### 示例 1:
<pre>
<strong>输入:</strong> "III"
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "IV"
<strong>输出:</strong> 4
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "IX"
<strong>输出:</strong> 9
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "LVIII"
<strong>输出:</strong> 58
<strong>解释:</strong> L = 50, V= 5, III = 3.
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> "MCMXCIV"
<strong>输出:</strong> 1994
<strong>解释:</strong> M = 1000, CM = 900, XC = 90 and IV = 4.
</pre>

## 题解 (Rust)

### 1. 字符串替换
```Rust
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.replace("IX", "IVV").replace("IV", "IIII");
        let s = s.replace("XC", "XLL").replace("XL", "XXXX");
        let s = s.replace("CM", "CDD").replace("CD", "CCCC");
 
        let mut n = 0;
        for c in s.chars() {
            match c {
                'I' => n += 1,
                'V' => n += 5,
                'X' => n += 10,
                'L' => n += 50,
                'C' => n += 100,
                'D' => n += 500,
                'M' => n += 1000,
                _ => (),
            };
        }
        n
    }
}
```

### 2. 查看下一个字符
```Rust
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut n = 0;
        let mut s = s.chars().peekable();
        while let Some(ch) = s.next() {
            match ch {
                'I' => match s.peek() {
                    Some('V') | Some('X') => n -= 1,
                    _ => n += 1,
                },
                'X' => match s.peek() {
                    Some('L') | Some('C') => n -= 10,
                    _ => n += 10,
                },
                'C' => match s.peek() {
                    Some('D') | Some('M') => n -= 100,
                    _ => n += 100,
                },
                'V' => n += 5,
                'L' => n += 50,
                'D' => n += 500,
                'M' => n += 1000,
                _ => (),
            };
        }
        n
    }
}
```

### 3. 减去差值
```Rust
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut n = 0;
        if s.contains("IV") { n -= 2; }
        if s.contains("IX") { n -= 2; }
        if s.contains("XL") { n -= 20; }
        if s.contains("XC") { n -= 20; }
        if s.contains("CD") { n -= 200; }
        if s.contains("CM") { n -= 200; }
 
        for ch in s.chars() {
            match ch {
                'I' => n += 1,
                'V' => n += 5,
                'X' => n += 10,
                'L' => n += 50,
                'C' => n += 100,
                'D' => n += 500,
                'M' => n += 1000,
                _ => (),
            };
        }
        n
    }
}
```
