# 380. O(1) 时间插入、删除和获取随机元素
实现`RandomizedSet` 类：

* `RandomizedSet()` 初始化 `RandomizedSet` 对象
* `bool insert(int val)` 当元素 `val` 不存在时，向集合中插入该项，并返回 `true` ；否则，返回 `false` 。
* `bool remove(int val)` 当元素 `val` 存在时，从集合中移除该项，并返回 `true` ；否则，返回 `false` 。
* `int getRandom()` 随机返回现有集合中的一项（测试用例保证调用此方法时集合中至少存在一个元素）。每个元素应该有 **相同的概率** 被返回。

你必须实现类的所有函数，并满足每个函数的 **平均** 时间复杂度为 `O(1)` 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
[[], [1], [2], [2], [], [1], [2], []]
<strong>输出:</strong>
[null, true, false, true, 2, true, false, 2]
<strong>解释:</strong>
RandomizedSet randomizedSet = new RandomizedSet();
randomizedSet.insert(1); // 向集合中插入 1 。返回 true 表示 1 被成功地插入。
randomizedSet.remove(2); // 返回 false ，表示集合中不存在 2 。
randomizedSet.insert(2); // 向集合中插入 2 。返回 true 。集合现在包含 [1,2] 。
randomizedSet.getRandom(); // getRandom 应随机返回 1 或 2 。
randomizedSet.remove(1); // 从集合中移除 1 ，返回 true 。集合现在包含 [2] 。
randomizedSet.insert(2); // 2 已在集合中，所以返回 false 。
randomizedSet.getRandom(); // 由于 2 是集合中唯一的数字，getRandom 总是返回 2 。
</pre>

#### 提示:
* <code>-2<sup>31</sup> <= val <= 2<sup>31</sup> - 1</code>
* 最多调用 `insert`、`remove` 和 `getRandom` 函数 <code>2 * 10<sup>5</sup></code> 次
* 在调用 `getRandom` 方法时，数据结构中 **至少存在一个** 元素。

## 题解 (Rust)

### 1. 题解
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
