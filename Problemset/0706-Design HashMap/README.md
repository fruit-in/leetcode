# 706. Design HashMap
Design a HashMap without using any built-in hash table libraries.

To be specific, your design should include these functions:
* <code>put(key, value)</code>: Insert a (key, value) pair into the HashMap. If the value already exists in the HashMap, update the value.
* <code>get(key)</code>: Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key.
* <code>remove(key)</code>: Remove the mapping for the value key if this map contains the mapping for the key.

#### Example:
<pre>
MyHashMap hashMap = new MyHashMap();
hashMap.put(1, 1);
hashMap.put(2, 2);
hashMap.get(1);            // returns 1
hashMap.get(3);            // returns -1 (not found)
hashMap.put(2, 1);          // update the existing value
hashMap.get(2);            // returns 1
hashMap.remove(2);          // remove the mapping for 2
hashMap.get(2);            // returns -1 (not found)
</pre>

#### Note:
* All keys and values will be in the range of <code>[0, 1000000]</code>.
* The number of operations will be in the range of <code>[1, 10000]</code>.
* Please do not use the built-in HashMap library.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
