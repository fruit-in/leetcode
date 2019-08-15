use std::collections::HashMap;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let map: HashMap<i32, usize> = arr2.iter().enumerate()
            .map(|(i, v)| (*v, i))
            .collect();
 
        let mut v1: Vec<i32> = Vec::new();
        let mut v2: Vec<i32> = Vec::new();
        for n in arr1 {
            match map.contains_key(&n) {
                true => v2.push(n),
                false => v1.push(n),
            };
        }
 
        v1.sort_unstable();
        v2.sort_unstable_by_key(|n| *map.get(&n).unwrap());
        v2.append(&mut v1);
        v2
    }
}
