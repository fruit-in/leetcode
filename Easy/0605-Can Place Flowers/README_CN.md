# 605. 种花问题
假设你有一个很长的花坛，一部分地块种植了花，另一部分却没有。可是，花卉不能种植在相邻的地块上，它们会争夺水源，两者都会死去。

给定一个花坛（表示为一个数组包含0和1，其中0表示没种植花，1表示种植了花），和一个数 **n** 。能否在不打破种植规则的情况下种入 **n** 朵花？能则返回True，不能则返回False。

#### 示例 1:
<pre>
<strong>输入:</strong> flowerbed = [1,0,0,0,1], n = 1
<strong>输出:</strong> True
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> flowerbed = [1,0,0,0,1], n = 2
<strong>输出:</strong> False
</pre>

#### 注意:
1. 数组内已种好的花不会违反种植规则。
2. 输入的数组长度范围为 [1, 20000]。
3. **n** 是非负整数，且不会超过输入数组的大小。

## 题解 (Rust)

### 1. 对连续的零计数
```Rust
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut zeroes = 1;

        for i in flowerbed {
            if i == 0 {
                zeroes += 1;
            } else {
                n -= (zeroes - 1) / 2;
                zeroes = 0;
            }
        }

        zeroes += 1;
        n -= (zeroes - 1) / 2;

        n <= 0
    }
}
```

### 2. 模拟
```Rust
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut n = n;
        let mut flowerbed = flowerbed;
        flowerbed.insert(0, 0);
        flowerbed.push(0);

        for i in 1..(flowerbed.len() - 1) {
            if flowerbed[i - 1] + flowerbed[i] + flowerbed[i + 1] == 0 {
                flowerbed[i] = 1;
                n -= 1;
            }
        }

        n <= 0
    }
}
```
