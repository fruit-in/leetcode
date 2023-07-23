# 475. 供暖器
冬季已经来临。 你的任务是设计一个有固定加热半径的供暖器向所有房屋供暖。

现在，给出位于一条水平线上的房屋和供暖器的位置，找到可以覆盖所有房屋的最小加热半径。

所以，你的输入将会是房屋和供暖器的位置。你将输出供暖器的最小加热半径。

#### 说明:
1. 给出的房屋和供暖器的数目是非负数且不会超过 25000。
2. 给出的房屋和供暖器的位置均是非负数且不会超过10^9。
3. 只要房屋位于供暖器的半径内(包括在边缘上)，它就可以得到供暖。
4. 所有供暖器都遵循你的半径标准，加热的半径也一样。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3],[2]
<strong>输出:</strong> 1
<strong>解释:</strong> 仅在位置2上有一个供暖器。如果我们将加热半径设为1，那么所有房屋就都能得到供暖。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,3,4],[1,4]
<strong>输出:</strong> 1
<strong>解释:</strong> 在位置1, 4上有两个供暖器。我们需要将加热半径设为1，这样所有房屋就都能得到供暖。
</pre>

## 题解 (Rust)

### 1. 暴力
```Rust
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut min_radius = 0;

        for house in &houses {
            let mut min_distance = std::i32::MAX;

            for heater in &heaters {
                min_distance = min_distance.min((heater - house).abs());
            }

            min_radius = min_radius.max(min_distance);
        }

        min_radius
    }
}
```

### 2. 双指针
```Rust
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort_unstable();
        heaters.sort_unstable();

        let mut i = 0;
        let mut min_radius = 0;

        for j in 0..houses.len() {
            while i < heaters.len() - 1 && heaters[i + 1] < houses[j] {
                i += 1;
            }

            let mut min_distance = (houses[j] - heaters[i]).abs();
            if i < heaters.len() - 1 {
                min_distance = min_distance.min(heaters[i + 1] - houses[j]);
            }

            min_radius = min_radius.max(min_distance);
        }

        min_radius
    }
}
```

### 3. 二分查找
```Rust
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut heaters = heaters;
        heaters.sort_unstable();

        let mut min_radius = 0;

        for house in houses {
            if let Err(i) = heaters.binary_search(&house) {
                min_radius = min_radius.max(
                    if i == 0 {
                        heaters[0] - house
                    } else if i == heaters.len() {
                        house - heaters[i - 1]
                    } else {
                        (heaters[i] - house).min(house - heaters[i - 1])
                    }
                );
            }
        }

        min_radius
    }
}
```
