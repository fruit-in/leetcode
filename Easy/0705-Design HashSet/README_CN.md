# 705. 设计哈希集合
不使用任何内建的哈希表库设计一个哈希集合

具体地说，你的设计应该包含以下的功能
* ```add(value)```：向哈希集合中插入一个值。
* ```contains(value)``` ：返回哈希集合中是否存在这个值。
* ```remove(value)```：将给定值从哈希集合中删除。如果哈希集合中没有这个值，什么也不做。

#### 示例:
<pre>
MyHashSet hashSet = new MyHashSet();
hashSet.add(1);
hashSet.add(2);
hashSet.contains(1);    // 返回 true
hashSet.contains(3);    // 返回 false (未找到)
hashSet.add(2);
hashSet.contains(2);    // 返回 true
hashSet.remove(2);
hashSet.contains(2);    // 返回 false (已经被删除)
</pre>

#### 注意:
* 所有的值都在 ```[0, 1000000]```的范围内。
* 操作的总数目在```[1, 10000]```范围内。
* 不要使用内建的哈希集合库。

## 题解 (Rust)

### 1. 题解
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
