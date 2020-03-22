# 1352. 最后 K 个数的乘积
请你实现一个「数字乘积类」```ProductOfNumbers```，要求支持下述两种方法：

1. ```add(int num)```
* 将数字 ```num``` 添加到当前数字列表的最后面。

2. ```getProduct(int k)```
* 返回当前数字列表中，最后 ```k``` 个数字的乘积。
* 你可以假设当前列表中始终 **至少** 包含 ```k``` 个数字。

题目数据保证：任何时候，任一连续数字序列的乘积都在 32-bit 整数范围内，不会溢出。

#### 示例:
<pre>
<strong>输入:</strong>
["ProductOfNumbers","add","add","add","add","add","getProduct","getProduct","getProduct","add","getProduct"]
[[],[3],[0],[2],[5],[4],[2],[3],[4],[8],[2]]
<strong>输出:</strong>
[null,null,null,null,null,null,20,40,0,null,32]
<strong>解释:</strong>
ProductOfNumbers productOfNumbers = new ProductOfNumbers();
productOfNumbers.add(3);        // [3]
productOfNumbers.add(0);        // [3,0]
productOfNumbers.add(2);        // [3,0,2]
productOfNumbers.add(5);        // [3,0,2,5]
productOfNumbers.add(4);        // [3,0,2,5,4]
productOfNumbers.getProduct(2); // 返回 20 。最后 2 个数字的乘积是 5 * 4 = 20
productOfNumbers.getProduct(3); // 返回 40 。最后 3 个数字的乘积是 2 * 5 * 4 = 40
productOfNumbers.getProduct(4); // 返回  0 。最后 4 个数字的乘积是 0 * 2 * 5 * 4 = 0
productOfNumbers.add(8);        // [3,0,2,5,4,8]
productOfNumbers.getProduct(2); // 返回 32 。最后 2 个数字的乘积是 4 * 8 = 32
</pre>

#### 提示:
* ```add``` 和 ```getProduct``` 两种操作加起来总共不会超过 ```40000``` 次。
* ```0 <= num <= 100```
* ```1 <= k <= 40000```

## 题解 (Rust)

### 1. 题解
```Rust
struct ProductOfNumbers {
    products: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

    fn new() -> Self {
        Self {
            products: vec![1],
        }
    }

    fn add(&mut self, num: i32) {
        match num {
            0 => self.products.truncate(1),
            _ => self.products.push(*self.products.last().unwrap() * num),
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let len = self.products.len();

        if k > len as i32 - 1 {
            return 0;
        }
        self.products[len - 1] / self.products[len - 1 - k as usize]
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
```
