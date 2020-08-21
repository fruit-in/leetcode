# 1342. 将数字变成 0 的操作次数
给你一个非负整数 ```num``` ，请你返回将它变成 0 所需要的步数。 如果当前数字是偶数，你需要把它除以 2 ；否则，减去 1 。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 14
<strong>输出:</strong> 6
<strong>解释:</strong>
步骤 1) 14 是偶数，除以 2 得到 7 。
步骤 2） 7 是奇数，减 1 得到 6 。
步骤 3） 6 是偶数，除以 2 得到 3 。
步骤 4） 3 是奇数，减 1 得到 2 。
步骤 5） 2 是偶数，除以 2 得到 1 。
步骤 6） 1 是奇数，减 1 得到 0 。
</pre>
  
#### 示例 2:
<pre>
<strong>输入:</strong> num = 8
<strong>输出:</strong> 4
<strong>解释:</strong>
步骤 1） 8 是偶数，除以 2 得到 4 。
步骤 2） 4 是偶数，除以 2 得到 2 。
步骤 3） 2 是偶数，除以 2 得到 1 。
步骤 4） 1 是奇数，减 1 得到 0 。
</pre>
  
#### 示例 3:
<pre>
<strong>输入:</strong> num = 123
<strong>输出:</strong> 12
</pre>

#### 提示:
* ```0 <= num <= 10^6```

## 题解 (Ruby)

### 1. 模拟
```Ruby
# @param {Integer} num
# @return {Integer}
def number_of_steps (num)
    ret = 0

    while num != 0
        if num % 2 == 0
            num /= 2
        else
            num -= 1
        end
        ret += 1
    end

    return ret
end
```

## 题解 (Rust)

### 1. 模拟
```Rust
impl Solution {
    pub fn number_of_steps (num: i32) -> i32 {
        let mut num = num;
        let mut ret = 0;

        while num != 0 {
            match num % 2 {
                0 => num /= 2,
                _ => num -= 1,
            }
            ret += 1;
        }

        ret
    }
}
```
