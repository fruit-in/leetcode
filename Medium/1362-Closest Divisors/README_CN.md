# 1362. 最接近的因数
给你一个整数 `num`，请你找出同时满足下面全部要求的两个整数：
* 两数乘积等于  `num + 1` 或 `num + 2`
* 以绝对差进行度量，两数大小最接近

你可以按任意顺序返回这两个整数。

#### 示例 1:
<pre>
<b>输入:</b> num = 8
<b>输出:</b> [3,3]
<b>解释:</b> 对于 num + 1 = 9，最接近的两个因数是 3 & 3；对于 num + 2 = 10, 最接近的两个因数是 2 & 5，因此返回 3 & 3 。
</pre>

#### 示例 2:
<pre>
<b>输入:</b> num = 123
<b>输出:</b> [5,25]
</pre>

#### 示例 3:
<pre>
<b>输入:</b> num = 999
<b>输出:</b> [40,25]
</pre>

#### 提示:
* `1 <= num <= 10^9`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} num
# @return {Integer[]}
def closest_divisors(num)
    ret = [1, num + 1]

    for n in (num + 1)..(num + 2)
        for i in (Integer.sqrt(n)...1).step(-1)
            if n % i == 0 and n / i - i < ret[1] - ret[0]
                ret = [i, n / i]
                break
            end
        end
    end

    return ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let mut ret = vec![1, num + 1];

        for n in (num + 1)..(num + 3) {
            for i in (2..=((n as f64).sqrt().floor() as i32)).rev() {
                if n % i == 0 && n / i - i < ret[1] - ret[0] {
                    ret = vec![i, n / i];
                    break;
                }
            }
        }

        ret
    }
}
```
