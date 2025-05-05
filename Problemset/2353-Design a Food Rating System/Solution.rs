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
