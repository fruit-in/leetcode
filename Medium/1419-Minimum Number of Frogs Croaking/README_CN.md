# 1419. 数青蛙
给你一个字符串 `croakOfFrogs`，它表示不同青蛙发出的蛙鸣声（字符串 "croak" ）的组合。由于同一时间可以有多只青蛙呱呱作响，所以 `croakOfFrogs` 中会混合多个 “croak” 。请你返回模拟字符串中所有蛙鸣所需不同青蛙的最少数目。

**注意:** 要想发出蛙鸣 "croak"，青蛙必须 **依序** 输出 `‘c’, ’r’, ’o’, ’a’, ’k’` 这 5 个字母。如果没有输出全部五个字母，那么它就不会发出声音。

如果字符串 `croakOfFrogs` 不是由若干有效的 "croak" 字符混合而成，请返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> croakOfFrogs = "croakcroak"
<strong>输出:</strong> 1
<strong>解释:</strong> 一只青蛙 “呱呱” 两次
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> croakOfFrogs = "crcoakroak"
<strong>输出:</strong> 2
<strong>解释:</strong> 最少需要两只青蛙，“呱呱” 声用黑体标注
第一只青蛙 "<b>cr</b>c<b>oak</b>roak".
第二只青蛙 "cr<b>c</b>oak<b>roak</b>".
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> croakOfFrogs = "croakcrook"
<strong>输出:</strong> -1
<strong>解释:</strong> 给出的字符串不是 "croak" 的有效组合。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> croakOfFrogs = "croakcroa"
<strong>输出:</strong> -1
</pre>

#### 提示:
* `1 <= croakOfFrogs.length <= 10^5`
* 字符串中的字符只有 `'c'`, `'r'`, `'o'`, `'a'` 或者 `'k'`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} croak_of_frogs
# @return {Integer}
def min_number_of_frogs(croak_of_frogs)
  counter = { 'c' => 0, 'r' => 0, 'o' => 0, 'a' => 0 }
  prev = { 'r' => 'c', 'o' => 'r', 'a' => 'o' }
  frogs = 0
  ret = 0

  croak_of_frogs.each_char do |c|
    if c == 'c'
      counter['c'] += 1
      frogs += 1
      ret = [ret, frogs].max
    elsif c == 'k'
      return -1 if counter['a'] == 0

      counter['a'] -= 1
      frogs -= 1
    else
      return -1 if counter[prev[c]] == 0

      counter[prev[c]] -= 1
      counter[c] += 1
    end
  end

  counter.values.all? { |v| v == 0 } ? ret : -1
end
```

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut counter = vec![('c', 0), ('r', 0), ('o', 0), ('a', 0)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let mut prev = vec![('r', 'c'), ('o', 'r'), ('a', 'o')]
            .into_iter()
            .collect::<HashMap<_, _>>();
        let mut frogs = 0;
        let mut ret = 0;

        for c in croak_of_frogs.chars() {
            match c {
                'c' => {
                    *counter.get_mut(&'c').unwrap() += 1;
                    frogs += 1;
                    ret = ret.max(frogs);
                }
                'k' => {
                    if counter[&'a'] == 0 {
                        return -1;
                    }
                    *counter.get_mut(&'a').unwrap() -= 1;
                    frogs -= 1;
                }
                _ => {
                    if counter[&prev[&c]] == 0 {
                        return -1;
                    }
                    *counter.get_mut(&prev[&c]).unwrap() -= 1;
                    *counter.get_mut(&c).unwrap() += 1;
                }
            }
        }

        if counter.values().all(|&v| v == 0) {
            ret
        } else {
            -1
        }
    }
}
```
