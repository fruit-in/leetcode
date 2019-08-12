struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 1000],
        }
    }
    
    /** value will always be non-negative. */
    fn put(&mut self, key: i32, value: i32) {
        let id = key as usize % 1000;
        match self.buckets[id].iter().position(|&x| x.0 == key) {
            Some(i) => self.buckets[id][i].1 = value,
            None => self.buckets[id].push((key, value)),
        };
    }
    
    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {
        let id = key as usize % 1000;
        for &(k, v) in &self.buckets[id] {
            if k == key {
                return v;
            }
        }
        -1
    }
    
    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&mut self, key: i32) {
        let id = key as usize % 1000;
        match self.buckets[id].iter().position(|&x| x.0 == key) {
            Some(i) => self.buckets[id].remove(i),
            None => (0, 0),
        };
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
