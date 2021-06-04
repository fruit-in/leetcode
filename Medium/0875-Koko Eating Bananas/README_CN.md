# 875. 爱吃香蕉的珂珂
珂珂喜欢吃香蕉。这里有 `N` 堆香蕉，第 `i` 堆中有 `piles[i]` 根香蕉。警卫已经离开了，将在 `H` 小时后回来。

珂珂可以决定她吃香蕉的速度 `K` （单位：根/小时）。每个小时，她将会选择一堆香蕉，从中吃掉 `K` 根。如果这堆香蕉少于 `K` 根，她将吃掉这堆的所有香蕉，然后这一小时内不会再吃更多的香蕉。

珂珂喜欢慢慢吃，但仍然想在警卫回来前吃掉所有的香蕉。

返回她可以在 `H` 小时内吃掉所有香蕉的最小速度 `K`（`K` 为整数）。

#### 示例 1:
<pre>
<strong>输入:</strong> piles = [3,6,7,11], H = 8
<strong>输出:</strong> 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> piles = [30,11,23,4,20], H = 5
<strong>输出:</strong> 30
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> piles = [30,11,23,4,20], H = 6
<strong>输出:</strong> 23
</pre>

#### 提示:
* `1 <= piles.length <= 10^4`
* `piles.length <= H <= 10^9`
* `1 <= piles[i] <= 10^9`

## 题解 (Ruby)

### 1. 二分查找
```Ruby
# @param {Integer[]} piles
# @param {Integer} h
# @return {Integer}
def min_eating_speed(piles, h)
  lo = 1
  hi = piles.max

  while lo < hi
    k = (lo + hi) / 2
    hours = piles.map { |x| (x - 1) / k + 1 }.sum

    if hours > h
      lo = k + 1
    else
      hi = k
    end
  end

  lo
end
```

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut lo = 1;
        let mut hi = *piles.iter().max().unwrap();

        while lo < hi {
            let k = (lo + hi) / 2;
            let hours = piles.iter().map(|&x| ((x - 1) / k + 1) as i64).sum::<i64>();

            if hours > h as i64 {
                lo = k + 1;
            } else {
                hi = k;
            }
        }

        lo
    }
}
```
