# 706. 设计哈希映射
不使用任何内建的哈希表库设计一个哈希映射

具体地说，你的设计应该包含以下的功能
* ```put(key, value)```：向哈希映射中插入(键,值)的数值对。如果键对应的值已经存在，更新这个值。
* ```get(key)```：返回给定的键所对应的值，如果映射中不包含这个键，返回-1。
* ```remove(key)```：如果映射中存在这个键，删除这个数值对。

#### 示例:
<pre>
MyHashMap hashMap = new MyHashMap();
hashMap.put(1, 1);
hashMap.put(2, 2);
hashMap.get(1);            // 返回 1
hashMap.get(3);            // 返回 -1 (未找到)
hashMap.put(2, 1);         // 更新已有的值
hashMap.get(2);            // 返回 1
hashMap.remove(2);         // 删除键为2的数据
hashMap.get(2);            // 返回 -1 (未找到)
</pre>

#### 注意:
* 所有的值都在 ```[1, 1000000]```的范围内。
* 操作的总数目在```[1, 10000]```范围内。
* 不要使用内建的哈希库。

## 题解 (Rust)

### 1. 题解
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
