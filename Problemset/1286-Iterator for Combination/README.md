# 1286. Iterator for Combination
Design an Iterator class, which has:
* A constructor that takes a string `characters` of **sorted distinct** lowercase English letters and a number `combinationLength` as arguments.
* A function *next()* that returns the next combination of length `combinationLength` in **lexicographical order**.
* A function *hasNext()* that returns `True` if and only if there exists a next combination.

#### Example:
```
CombinationIterator iterator = new CombinationIterator("abc", 2); // creates the iterator.

iterator.next(); // returns "ab"
iterator.hasNext(); // returns true
iterator.next(); // returns "ac"
iterator.hasNext(); // returns true
iterator.next(); // returns "bc"
iterator.hasNext(); // returns false
```

#### Constraints:
* `1 <= combinationLength <= characters.length <= 15`
* There will be at most `10^4` function calls per test.
* It's guaranteed that all calls of the function `next` are valid.

## Solutions (Rust)

### 1. Solution
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
