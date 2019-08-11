struct MyHashSet {
    buckets: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 1000],
        }
    }
    
    fn add(&mut self, key: i32) {
        let id = key as usize % 1000;
        if !self.buckets[id].contains(&key) {
            self.buckets[id].push(key);
        }
    }
    
    fn remove(&mut self, key: i32) {
        let id = key as usize % 1000;
        match self.buckets[id].iter().position(|&x| x == key) {
            Some(i) => self.buckets[id].remove(i),
            None => 0,
        };
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        let id = key as usize % 1000;
        self.buckets[id].contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
