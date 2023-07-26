use std::collections::HashMap;

struct RangeFreqQuery {
    indices: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut indices = HashMap::new();

        for i in 0..arr.len() {
            indices
                .entry(arr[i])
                .and_modify(|v: &mut Vec<i32>| v.push(i as i32))
                .or_insert(vec![i as i32]);
        }

        Self { indices }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if !self.indices.contains_key(&value) {
            return 0;
        }

        let indices = self.indices.get(&value).unwrap();
        let left = match indices.binary_search(&left) {
            Ok(i) | Err(i) => i as i32,
        };
        let right = match indices.binary_search(&right) {
            Ok(i) => i as i32,
            Err(i) => i as i32 - 1,
        };

        right - left + 1
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
