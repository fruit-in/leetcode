# 1276. 不浪费原料的汉堡制作方案
圣诞活动预热开始啦，汉堡店推出了全新的汉堡套餐。为了避免浪费原料，请你帮他们制定合适的制作计划。

给你两个整数 `tomatoSlices` 和 `cheeseSlices`，分别表示番茄片和奶酪片的数目。不同汉堡的原料搭配如下：
* **巨无霸汉堡:** 4 片番茄和 1 片奶酪
* **小皇堡:** 2 片番茄和 1 片奶酪

请你以 `[total_jumbo, total_small]`（[巨无霸汉堡总数，小皇堡总数]）的格式返回恰当的制作方案，使得剩下的番茄片 `tomatoSlices` 和奶酪片 `cheeseSlices` 的数量都是 `0`。

如果无法使剩下的番茄片 `tomatoSlices` 和奶酪片 `cheeseSlices` 的数量为 `0`，就请返回 `[]`。

#### 示例 1:
<pre>
<strong>输入:</strong> tomatoSlices = 16, cheeseSlices = 7
<strong>输出:</strong> [1,6]
<strong>解释:</strong> 制作 1 个巨无霸汉堡和 6 个小皇堡需要 4*1 + 2*6 = 16 片番茄和 1 + 6 = 7 片奶酪。不会剩下原料。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tomatoSlices = 17, cheeseSlices = 4
<strong>输出:</strong> []
<strong>解释:</strong> 只制作小皇堡和巨无霸汉堡无法用光全部原料。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> tomatoSlices = 4, cheeseSlices = 17
<strong>输出:</strong> []
<strong>解释:</strong> 制作 1 个巨无霸汉堡会剩下 16 片奶酪，制作 2 个小皇堡会剩下 15 片奶酪。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> tomatoSlices = 0, cheeseSlices = 0
<strong>输出:</strong> [0,0]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> tomatoSlices = 2, cheeseSlices = 1
<strong>输出:</strong> [0,1]
</pre>

#### 提示:
* `0 <= tomatoSlices <= 10^7`
* `0 <= cheeseSlices <= 10^7`

## 题解 (Python)

### 1. 数学
```Python
class Solution:
    def numOfBurgers(self, tomatoSlices: int, cheeseSlices: int) -> List[int]:
        if tomatoSlices % 2 == 0 \
                and tomatoSlices >= 2 * cheeseSlices \
                and tomatoSlices <= 4 * cheeseSlices:
            return [
                tomatoSlices // 2 - cheeseSlices,
                2 * cheeseSlices - tomatoSlices // 2
            ]
        return []
```

## 题解 (Rust)

### 1. 数学
```Rust
impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if tomato_slices % 2 == 0
            && tomato_slices >= 2 * cheese_slices
            && tomato_slices <= 4 * cheese_slices
        {
            vec![
                tomato_slices / 2 - cheese_slices,
                2 * cheese_slices - tomato_slices / 2,
            ]
        } else {
            vec![]
        }
    }
}
```
