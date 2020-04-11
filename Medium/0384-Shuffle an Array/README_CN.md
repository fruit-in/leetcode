# 384. 打乱数组
打乱一个没有重复元素的数组。

#### 示例:
```
// 以数字集合 1, 2 和 3 初始化数组。
int[] nums = {1,2,3};
Solution solution = new Solution(nums);

// 打乱数组 [1,2,3] 并返回结果。任何 [1,2,3]的排列返回的概率应该相同。
solution.shuffle();

// 重设数组到它的初始状态[1,2,3]。
solution.reset();

// 随机返回数组[1,2,3]打乱后的结果。
solution.shuffle();
```

## 题解 (Rust)

### 1. 题解
```Rust
use rand::seq::SliceRandom;

struct Solution {
    original: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        Self {
            original: nums,
        }
    }

    /** Resets the array to its original configuration and return it. */
    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    /** Returns a random shuffling of the array. */
    fn shuffle(&self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let mut array = self.original.clone();
        array.shuffle(&mut rng);
        array
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
```
