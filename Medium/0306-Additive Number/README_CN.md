# 306. 累加数
累加数是一个字符串，组成它的数字可以形成累加序列。

一个有效的累加序列必须**至少**包含 3 个数。除了最开始的两个数以外，字符串中的其他数都等于它之前两个数相加的和。

给定一个只包含数字 `'0'-'9'` 的字符串，编写一个算法来判断给定输入是否是累加数。

**说明:** 累加序列里的数不会以 0 开头，所以不会出现 `1, 2, 03` 或者 `1, 02, 3` 的情况。

#### 示例 1:
<pre>
<strong>输入:</strong> "112358"
<strong>输出:</strong> true
<strong>解释:</strong> 累加序列为: 1, 1, 2, 3, 5, 8 。1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "199100199"
<strong>输出:</strong> true
<strong>解释:</strong> 累加序列为: 1, 99, 100, 199。1 + 99 = 100, 99 + 100 = 199
</pre>

#### 进阶:
你如何处理一个溢出的过大的整数输入?

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {String} num
# @return {Boolean}
def is_additive_number(num)
  (1...(num.length + 1) / 2).each do |i|
    break if num[0] == '0' && i > 1

    (1..[(num.length - i) / 2, num.length - 2 * i].min).each do |j|
      break if num[i] == '0' && j > 1

      k = i + j
      x = num[0...i].to_i
      y = num[i...k].to_i
      l = k + (x + y).to_s.length

      while l <= num.length
        z = num[k...l].to_i

        break if x + y != z

        x = y
        y = z
        k = l
        l = k + (x + y).to_s.length

        return true if k == num.length
      end
    end
  end

  false
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        for i in 1..(num.len() + 1) / 2 {
            if num.get(..1).unwrap() == "0" && i > 1 {
                break;
            }

            for j in 1..=((num.len() - i) / 2).min(num.len() - 2 * i) {
                if num.get(i..i + 1).unwrap() == "0" && j > 1 {
                    break;
                }

                let mut k = i + j;
                let mut x = num.get(..i).unwrap().parse::<u64>().unwrap();
                let mut y = num.get(i..k).unwrap().parse::<u64>().unwrap();
                let mut l = k + (x + y).to_string().len();
                let mut z;

                while l <= num.len() {
                    z = num.get(k..l).unwrap().parse::<u64>().unwrap();

                    if x + y != z {
                        break;
                    }

                    x = y;
                    y = z;
                    k = l;
                    l = k + (x + y).to_string().len();
                }

                if k == num.len() {
                    return true;
                }
            }
        }

        false
    }
}
```
