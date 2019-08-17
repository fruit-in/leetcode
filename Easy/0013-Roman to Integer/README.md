# 13. Roman to Integer
Roman numerals are represented by seven different symbols: ```I```, ```V```, ```X```, ```L```, ```C```, ```D``` and ```M```.

<pre>
<strong>Symbol</strong>       <strong>Value</strong>
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
</pre>

For example, two is written as ```II``` in Roman numeral, just two one's added together. Twelve is written as, ```XII```, which is simply ```X``` + ```II```. The number twenty seven is written as ```XXVII```, which is ```XX``` + ```V``` + ```II```.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not ```IIII```. Instead, the number four is written as ```IV```. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as ```IX```. There are six instances where subtraction is used:

* ```I``` can be placed before ```V``` (5) and ```X``` (10) to make 4 and 9.
* ```X``` can be placed before ```L``` (50) and ```C``` (100) to make 40 and 90.
* ```C``` can be placed before ```D``` (500) and ```M``` (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer. Input is guaranteed to be within the range from 1 to 3999.

#### Example 1:
<pre>
<strong>Input:</strong> "III"
<strong>Output:</strong> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "IV"
<strong>Output:</strong> 4
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "IX"
<strong>Output:</strong> 9
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> "LVIII"
<strong>Output:</strong> 58
<strong>Explanation:</strong> L = 50, V= 5, III = 3.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> "MCMXCIV"
<strong>Output:</strong> 1994
<strong>Explanation:</strong> M = 1000, CM = 900, XC = 90 and IV = 4.
</pre>

## Solutions (Rust)

### 1. Replace
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

### 2. Check Next
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

### 3. Subtract Difference
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
