# 475. Heaters
Winter is coming! Your first job during the contest is to design a standard heater with fixed warm radius to warm all the houses.

Now, you are given positions of houses and heaters on a horizontal line, find out minimum radius of heaters so that all houses could be covered by those heaters.

So, your input will be the positions of houses and heaters seperately, and your expected output will be the minimum radius standard of heaters.

#### Note:
1. Numbers of houses and heaters you are given are non-negative and will not exceed 25000.
2. Positions of houses and heaters you are given are non-negative and will not exceed 10^9.
3. As long as a house is in the heaters' warm radius range, it can be warmed.
4. All the heaters follow your radius standard and the warm radius will the same.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3],[2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The only heater was placed in the position 2, and if we use the radius 1 standard, then all the houses can be warmed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,3,4],[1,4]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The two heater was placed in the position 1 and 4. We need to use radius 1 standard, then all the houses can be warmed.
</pre>

## Solutions (Rust)

### 1. Brute Force
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

### 2. Two Pointers
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

### 3. Binary Search
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
