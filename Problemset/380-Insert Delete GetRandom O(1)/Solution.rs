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
