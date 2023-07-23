# 1286. 字母组合迭代器
请你设计一个迭代器类，包括以下内容：
* 一个构造函数，输入参数包括：一个 **有序且字符唯一** 的字符串 `characters`（该字符串只包含小写英文字母）和一个数字 `combinationLength` 。
* 函数 *next()* ，按 **字典序** 返回长度为 `combinationLength` 的下一个字母组合。
* 函数 *hasNext()* ，只有存在长度为 `combinationLength` 的下一个字母组合时，才返回 `True`；否则，返回 `False`。

#### 示例:
```
CombinationIterator iterator = new CombinationIterator("abc", 2); // 创建迭代器 iterator

iterator.next(); // 返回 "ab"
iterator.hasNext(); // 返回 true
iterator.next(); // 返回 "ac"
iterator.hasNext(); // 返回 true
iterator.next(); // 返回 "bc"
iterator.hasNext(); // 返回 false
```

#### 提示:
* `1 <= combinationLength <= characters.length <= 15`
* 每组测试数据最多包含 `10^4` 次函数调用。
* 题目保证每次调用函数 `next` 时都存在下一个字母组合。

## 题解 (Rust)

### 1. 题解
```Rust
struct CombinationIterator {
    chars: Vec<u8>,
    combins: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        Self {
            chars: characters.clone().into_bytes(),
            combins: (0..2_i32.pow(characters.len() as u32))
                .filter(|x| x.count_ones() == combinationLength as u32)
                .collect(),
        }
    }

    fn next(&mut self) -> String {
        let mask = self.combins.pop().unwrap();
        let mut ret = vec![];

        for i in (0..self.chars.len()).rev() {
            if mask & (1 << i) != 0 {
                ret.push(self.chars[self.chars.len() - 1 - i])
            }
        }

        String::from_utf8(ret).unwrap()
    }

    fn has_next(&self) -> bool {
        !self.combins.is_empty()
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */
```
