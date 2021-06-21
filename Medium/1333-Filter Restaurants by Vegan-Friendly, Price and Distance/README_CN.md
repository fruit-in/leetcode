# 1333. 餐厅过滤器
给你一个餐馆信息数组 `restaurants`，其中  <code>restaurants[i] = [id<sub>i</sub>, rating<sub>i</sub>, veganFriendly<sub>i</sub>, price<sub>i</sub>, distance<sub>i</sub>]</code>。你必须使用以下三个过滤器来过滤这些餐馆信息。

其中素食者友好过滤器 `veganFriendly` 的值可以为 `true` 或者 `false`，如果为 *true* 就意味着你应该只包括 <code>veganFriendly<sub>i</sub></code> 为 true 的餐馆，为 *false* 则意味着可以包括任何餐馆。此外，我们还有最大价格 `maxPrice` 和最大距离 `maxDistance` 两个过滤器，它们分别考虑餐厅的价格因素和距离因素的最大值。

过滤后返回餐馆的 ***id***，按照 ***rating*** 从高到低排序。如果 ***rating*** 相同，那么按 ***id*** 从高到低排序。简单起见， <code>veganFriendly<sub>i</sub></code> 和 `veganFriendly` 为 *true* 时取值为 *1*，为 *false* 时，取值为 *0* 。

#### 示例 1:
<pre>
<strong>输入:</strong> restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 1, maxPrice = 50, maxDistance = 10
<strong>输出:</strong> [3,1,5]
<strong>解释:</strong>
这些餐馆为：
餐馆 1 [id=1, rating=4, veganFriendly=1, price=40, distance=10]
餐馆 2 [id=2, rating=8, veganFriendly=0, price=50, distance=5]
餐馆 3 [id=3, rating=8, veganFriendly=1, price=30, distance=4]
餐馆 4 [id=4, rating=10, veganFriendly=0, price=10, distance=3]
餐馆 5 [id=5, rating=1, veganFriendly=1, price=15, distance=1]
在按照 veganFriendly = 1, maxPrice = 50 和 maxDistance = 10 进行过滤后，我们得到了餐馆 3, 餐馆 1 和 餐馆 5（按评分从高到低排序）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 0, maxPrice = 50, maxDistance = 10
<strong>输出:</strong> [4,3,2,1,5]
<strong>解释:</strong> 餐馆与示例 1 相同，但在 veganFriendly = 0 的过滤条件下，应该考虑所有餐馆。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> restaurants = [[1,4,1,40,10],[2,8,0,50,5],[3,8,1,30,4],[4,10,0,10,3],[5,1,1,15,1]], veganFriendly = 0, maxPrice = 30, maxDistance = 3
<strong>输出:</strong> [4,5]
</pre>

#### 提示:
* `1 <= restaurants.length <= 10^4`
* `restaurants[i].length == 5`
* <code>1 <= id<sub>i</sub>, rating<sub>i</sub>, price<sub>i</sub>, distance<sub>i</sub> <= 10^5</code>
* `1 <= maxPrice, maxDistance <= 10^5`
* <code>veganFriendly<sub>i</sub></code> 和 `veganFriendly` 的值为 0 或 1 。
* 所有 <code>id<sub>i</sub></code> 各不相同。

## 题解 (Ruby)

### 1. 排序
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

## 题解 (Rust)

### 1. 排序
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
