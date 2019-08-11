# 705. Design HashSet
Design a HashSet without using any built-in hash table libraries.

To be specific, your design should include these functions:
* <code>add(value)</code>: Insert a value into the HashSet.
* <code>contains(value)</code>: Return whether the value exists in the HashSet or not.
* <code>remove(value)</code>: Remove a value in the HashSet. If the value does not exist in the HashSet, do nothing.

#### Example:
<pre>
MyHashSet hashSet = new MyHashSet();
hashSet.add(1);
hashSet.add(2);
hashSet.contains(1);    // returns true
hashSet.contains(3);    // returns false (not found)
hashSet.add(2);
hashSet.contains(2);    // returns true
hashSet.remove(2);
hashSet.contains(2);    // returns false (already removed)
</pre>

#### Note:
* All values will be in the range of <code>[0, 1000000]</code>.
* The number of operations will be in the range of <code>[1, 10000]</code>.
* Please do not use the built-in HashSet library.

## Solutions (Python)

### 1. Solution
```Rust
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
```
