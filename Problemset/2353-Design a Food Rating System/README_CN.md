# 2353. 设计食物评分系统
设计一个支持下述操作的食物评分系统：
* **修改** 系统中列出的某种食物的评分。
* 返回系统中某一类烹饪方式下评分最高的食物。

实现 `FoodRatings` 类：
* `FoodRatings(String[] foods, String[] cuisines, int[] ratings)` 初始化系统。食物由 `foods`、`cuisines` 和 `ratings` 描述，长度均为 `n` 。
    * `foods[i]` 是第 `i` 种食物的名字。
    * `cuisines[i]` 是第 `i` 种食物的烹饪方式。
    * `ratings[i]` 是第 `i` 种食物的最初评分。
* `void changeRating(String food, int newRating)` 修改名字为 `food` 的食物的评分。
* `String highestRated(String cuisine)` 返回指定烹饪方式 `cuisine` 下评分最高的食物的名字。如果存在并列，返回 **字典序较小** 的名字。

注意，字符串 `x` 的字典序比字符串 `y` 更小的前提是：`x` 在字典中出现的位置在 `y` 之前，也就是说，要么 `x` 是 `y` 的前缀，或者在满足 `x[i] != y[i]` 的第一个位置 `i` 处，`x[i]` 在字母表中出现的位置在 `y[i]` 之前。

#### 示例 1:
<pre>
<strong>输入:</strong>
["FoodRatings", "highestRated", "highestRated", "changeRating", "highestRated", "changeRating", "highestRated"]
[[["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]], ["korean"], ["japanese"], ["sushi", 16], ["japanese"], ["ramen", 16], ["japanese"]]
<strong>输出:</strong>
[null, "kimchi", "ramen", null, "sushi", null, "ramen"]
<strong>解释:</strong>
FoodRatings foodRatings = new FoodRatings(["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]);
foodRatings.highestRated("korean"); // 返回 "kimchi"
                                    // "kimchi" 是分数最高的韩式料理，评分为 9 。
foodRatings.highestRated("japanese"); // 返回 "ramen"
                                      // "ramen" 是分数最高的日式料理，评分为 14 。
foodRatings.changeRating("sushi", 16); // "sushi" 现在评分变更为 16 。
foodRatings.highestRated("japanese"); // 返回 "sushi"
                                      // "sushi" 是分数最高的日式料理，评分为 16 。
foodRatings.changeRating("ramen", 16); // "ramen" 现在评分变更为 16 。
foodRatings.highestRated("japanese"); // 返回 "ramen"
                                      // "sushi" 和 "ramen" 的评分都是 16 。
                                      // 但是，"ramen" 的字典序比 "sushi" 更小。
</pre>

#### 提示:
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* `n == foods.length == cuisines.length == ratings.length`
* `1 <= foods[i].length, cuisines[i].length <= 10`
* `foods[i]`、`cuisines[i]` 由小写英文字母组成
* <code>1 <= ratings[i] <= 10<sup>8</sup></code>
* `foods` 中的所有字符串 **互不相同**
* 在对 `changeRating` 的所有调用中，`food` 是系统中食物的名字。
* 在对 `highestRated` 的所有调用中，`cuisine` 是系统中 **至少一种** 食物的烹饪方式。
* 最多调用 `changeRating` 和 `highestRated` **总计** <code>2 * 10<sup>4</sup></code> 次

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct FoodRatings {
    food_rating: HashMap<String, i32>,
    food_cuisine: HashMap<String, String>,
    highest_rating: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_rating = HashMap::new();
        let mut food_cuisine = HashMap::new();
        let mut highest_rating = HashMap::new();

        for i in 0..foods.len() {
            food_rating.insert(foods[i].clone(), ratings[i]);
            food_cuisine.insert(foods[i].clone(), cuisines[i].clone());
            highest_rating
                .entry(cuisines[i].clone())
                .or_insert(BinaryHeap::new())
                .push((ratings[i], Reverse(foods[i].clone())));
        }

        Self {
            food_rating,
            food_cuisine,
            highest_rating,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        *self.food_rating.get_mut(&food).unwrap() = new_rating;
        self.highest_rating
            .get_mut(&self.food_cuisine[&food])
            .unwrap()
            .push((new_rating, Reverse(food)));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let mut heap = self.highest_rating.get_mut(&cuisine).unwrap();

        while let Some(&(rating, Reverse(ref food))) = heap.peek() {
            if self.food_rating[food] == rating {
                return food.to_string();
            } else {
                heap.pop();
            }
        }

        unreachable!();
    }
}

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
```
