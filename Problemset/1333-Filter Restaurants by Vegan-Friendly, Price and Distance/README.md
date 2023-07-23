# 1333. Filter Restaurants by Vegan-Friendly, Price and Distance
Given the array `restaurants` where  <code>restaurants[i] = [id<sub>i</sub>, rating<sub>i</sub>, veganFriendly<sub>i</sub>, price<sub>i</sub>, distance<sub>i</sub>]</code>. You have to filter the restaurants using three filters.

The `veganFriendly` filter will be either *true* (meaning you should only include restaurants with <code>veganFriendly<sub>i</sub></code> set to true) or *false* (meaning you can include any restaurant). In addition, you have the filters `maxPrice` and `maxDistance` which are the maximum value for price and distance of restaurants you should consider respectively.

Return the array of restaurant ***IDs*** after filtering, ordered by **rating** from highest to lowest. For restaurants with the same rating, order them by ***id*** from highest to lowest. For simplicity <code>veganFriendly<sub>i</sub></code> and `veganFriendly` take value *1* when it is *true*, and *0* when it is *false*.

#### Example 1:
<pre>
<strong>Input:</strong> restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 1, maxPrice = 50, maxDistance = 10
<strong>Output:</strong> [3,1,5]
<strong>Explanation:</strong>
The restaurants are:
Restaurant 1 [id=1, rating=4, veganFriendly=1, price=40, distance=10]
Restaurant 2 [id=2, rating=8, veganFriendly=0, price=50, distance=5]
Restaurant 3 [id=3, rating=8, veganFriendly=1, price=30, distance=4]
Restaurant 4 [id=4, rating=10, veganFriendly=0, price=10, distance=3]
Restaurant 5 [id=5, rating=1, veganFriendly=1, price=15, distance=1]
After filter restaurants with veganFriendly = 1, maxPrice = 50 and maxDistance = 10 we have restaurant 3, restaurant 1 and restaurant 5 (ordered by rating from highest to lowest).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 0, maxPrice = 50, maxDistance = 10
<strong>Output:</strong> [4,3,2,1,5]
<strong>Explanation:</strong> The restaurants are the same as in example 1, but in this case the filter veganFriendly = 0, therefore all restaurants are considered.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 0, maxPrice = 30, maxDistance = 3
<strong>Output:</strong> [4,5]
</pre>

#### Constraints:
* `1 <= restaurants.length <= 10^4`
* `restaurants[i].length == 5`
* <code>1 <= id<sub>i</sub>, rating<sub>i</sub>, price<sub>i</sub>, distance<sub>i</sub> <= 10^5</code>
* `1 <= maxPrice, maxDistance <= 10^5`
* <code>veganFriendly<sub>i</sub></code> and `veganFriendly` are 0 or 1.
* All <code>id<sub>i</sub></code> are distinct.

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[][]} restaurants
# @param {Integer} vegan_friendly
# @param {Integer} max_price
# @param {Integer} max_distance
# @return {Integer[]}
def filter_restaurants(restaurants, vegan_friendly, max_price, max_distance)
  restaurants.filter do |r|
    r[2] >= vegan_friendly && r[3] <= max_price && r[4] <= max_distance
  end.sort_by { |r| [-r[1], -r[0]] }.map { |r| r[0] }
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn filter_restaurants(
        restaurants: Vec<Vec<i32>>,
        vegan_friendly: i32,
        max_price: i32,
        max_distance: i32,
    ) -> Vec<i32> {
        let mut restaurants = restaurants
            .iter()
            .filter(|r| r[2] >= vegan_friendly && r[3] <= max_price && r[4] <= max_distance)
            .collect::<Vec<_>>();
        restaurants.sort_unstable_by_key(|r| (-r[1], -r[0]));

        restaurants.iter().map(|r| r[0]).collect()
    }
}
```
