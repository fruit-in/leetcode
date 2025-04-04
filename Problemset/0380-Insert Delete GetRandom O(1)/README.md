# 380. Insert Delete GetRandom O(1)
Implement the `RandomizedSet` class:

* `RandomizedSet()` Initializes the `RandomizedSet` object.
* `bool insert(int val)` Inserts an item `val` into the set if not present. Returns `true` if the item was not present, `false` otherwise.
* `bool remove(int val)` Removes an item `val` from the set if present. Returns `true` if the item was present, `false` otherwise.
* `int getRandom()` Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the **same probability** of being returned.

You must implement the functions of the class such that each function works in **average** `O(1)` time complexity.

#### Example 1:
<pre>
<strong>Input:</strong>
["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
[[], [1], [2], [2], [], [1], [2], []]
<strong>Output:</strong>
[null, true, false, true, 2, true, false, 2]
<strong>Explanation:</strong>
RandomizedSet randomizedSet = new RandomizedSet();
randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
randomizedSet.insert(2); // 2 was already in the set, so return false.
randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
</pre>

#### Constraints:
* <code>-2<sup>31</sup> <= val <= 2<sup>31</sup> - 1</code>
* At most <code>2 * 10<sup>5</sup></code> calls will be made to `insert`, `remove`, and `getRandom`.
* There will be **at least one** element in the data structure when `getRandom` is called.

## Solutions (Rust)

### 1. Solution
```Rust
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

struct RandomizedSet {
    vals_map: HashMap<i32, usize>,
    vals_vec: Vec<i32>,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            vals_map: HashMap::new(),
            vals_vec: vec![],
            rng: thread_rng(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if !self.vals_map.contains_key(&val) {
            self.vals_map.insert(val, self.vals_vec.len());
            self.vals_vec.push(val);

            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if self.vals_map.contains_key(&val) {
            self.vals_map
                .insert(*self.vals_vec.last().unwrap(), self.vals_map[&val]);
            self.vals_vec
                .swap_remove(self.vals_map.remove(&val).unwrap());

            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        *self.vals_vec.choose(&mut self.rng).unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
```
