# 1431. Kids With the Greatest Number of Candies
Given the array `candies` and the integer `extraCandies`, where `candies[i]` represents the number of candies that the ***ith*** kid has.

For each kid check if there is a way to distribute `extraCandies` among the kids such that he or she can have the **greatest** number of candies among them. Notice that multiple kids can have the **greatest** number of candies.

#### Example 1:
<pre>
<strong>Input:</strong> candies = [2,3,5,1,3], extraCandies = 3
<strong>Output:</strong> [true,true,true,false,true]
<strong>Explanation:</strong>
Kid 1 has 2 candies and if he or she receives all extra candies (3) will have 5 candies --- the greatest number of candies among the kids.
Kid 2 has 3 candies and if he or she receives at least 2 extra candies will have the greatest number of candies among the kids.
Kid 3 has 5 candies and this is already the greatest number of candies among the kids.
Kid 4 has 1 candy and even if he or she receives all extra candies will only have 4 candies.
Kid 5 has 3 candies and if he or she receives at least 2 extra candies will have the greatest number of candies among the kids.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> candies = [4,2,1,1,2], extraCandies = 1
<strong>Output:</strong> [true,false,false,false,false]
<strong>Explanation:</strong> There is only 1 extra candy, therefore only kid 1 will have the greatest number of candies among the kids regardless of who takes the extra candy.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> candies = [12,1,12], extraCandies = 10
<strong>Output:</strong> [true,false,true]
</pre>

#### Constraints:
* `2 <= candies.length <= 100`
* `1 <= candies[i] <= 100`
* `1 <= extraCandies <= 50`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let greatest = *candies.iter().max().unwrap();

        candies.iter().map(|x| x + extra_candies >= greatest).collect()
    }
}
```
