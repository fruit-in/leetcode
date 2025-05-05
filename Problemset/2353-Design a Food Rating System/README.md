# 2353. Design a Food Rating System
Design a food rating system that can do the following:
* **Modify** the rating of a food item listed in the system.
* Return the highest-rated food item for a type of cuisine in the system.

Implement the `FoodRatings` class:
* `FoodRatings(String[] foods, String[] cuisines, int[] ratings)` Initializes the system. The food items are described by `foods`, `cuisines` and `ratings`, all of which have a length of `n`.
    * `foods[i]` is the name of the <code>i<sup>th</sup></code> food,
    * `cuisines[i]` is the type of cuisine of the <code>i<sup>th</sup></code> food, and
    * `ratings[i]` is the initial rating of the <code>i<sup>th</sup></code> food.
* `void changeRating(String food, int newRating)` Changes the rating of the food item with the name `food`.
* `String highestRated(String cuisine)` Returns the name of the food item that has the highest rating for the given type of `cuisine`. If there is a tie, return the item with the **lexicographically smaller** name.

Note that a string `x` is lexicographically smaller than string `y` if `x` comes before `y` in dictionary order, that is, either `x` is a prefix of `y`, or if `i` is the first position such that `x[i] != y[i]`, then `x[i]` comes before `y[i]` in alphabetic order.

#### Example 1:
<pre>
<strong>Input:</strong>
["FoodRatings", "highestRated", "highestRated", "changeRating", "highestRated", "changeRating", "highestRated"]
[[["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]], ["korean"], ["japanese"], ["sushi", 16], ["japanese"], ["ramen", 16], ["japanese"]]
<strong>Output:</strong>
[null, "kimchi", "ramen", null, "sushi", null, "ramen"]
<strong>Explanation:</strong>
FoodRatings foodRatings = new FoodRatings(["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"], ["korean", "japanese", "japanese", "greek", "japanese", "korean"], [9, 12, 8, 15, 14, 7]);
foodRatings.highestRated("korean"); // return "kimchi"
                                    // "kimchi" is the highest rated korean food with a rating of 9.
foodRatings.highestRated("japanese"); // return "ramen"
                                      // "ramen" is the highest rated japanese food with a rating of 14.
foodRatings.changeRating("sushi", 16); // "sushi" now has a rating of 16.
foodRatings.highestRated("japanese"); // return "sushi"
                                      // "sushi" is the highest rated japanese food with a rating of 16.
foodRatings.changeRating("ramen", 16); // "ramen" now has a rating of 16.
foodRatings.highestRated("japanese"); // return "ramen"
                                      // Both "sushi" and "ramen" have a rating of 16.
                                      // However, "ramen" is lexicographically smaller than "sushi".
</pre>

#### Constraints:
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* `n == foods.length == cuisines.length == ratings.length`
* `1 <= foods[i].length, cuisines[i].length <= 10`
* `foods[i]`, `cuisines[i]` consist of lowercase English letters.
* <code>1 <= ratings[i] <= 10<sup>8</sup></code>
* All the strings in `foods` are **distinct**.
* `food` will be the name of a food item in the system across all calls to `changeRating`.
* `cuisine` will be a type of cuisine of **at least one** food item in the system across all calls to `highestRated`.
* At most <code>2 * 10<sup>4</sup></code> calls **in total** will be made to `changeRating` and `highestRated`.

## Solutions (Rust)

### 1. Solution
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
