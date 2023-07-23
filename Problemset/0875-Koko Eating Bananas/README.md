# 875. Koko Eating Bananas
Koko loves to eat bananas. There are `n` piles of bananas, the <code>i<sup>th</sup></code> pile has `piles[i]` bananas. The guards have gone and will come back in `h` hours.

Koko can decide her bananas-per-hour eating speed of `k`. Each hour, she chooses some pile of bananas and eats `k` bananas from that pile. If the pile has less than `k` bananas, she eats all of them instead and will not eat any more bananas during this hour.

Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

Return *the minimum integer* `k` *such that she can eat all the bananas within* `h` *hours*.

#### Example 1:
<pre>
<strong>Input:</strong> piles = [3,6,7,11], h = 8
<strong>Output:</strong> 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> piles = [30,11,23,4,20], h = 5
<strong>Output:</strong> 30
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> piles = [30,11,23,4,20], h = 6
<strong>Output:</strong> 23
</pre>

#### Constraints:
* <code>1 <= piles.length <= 10<sup>4</sup></code>
* <code>piles.length <= h <= 10<sup>9</sup></code>
* <code>1 <= piles[i] <= 10<sup>9</sup></code>

## Solutions (Ruby)

### 1. Binary Search
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

## Solutions (Rust)

### 1. Binary Search
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
