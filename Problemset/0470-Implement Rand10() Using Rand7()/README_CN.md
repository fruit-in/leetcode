# 470. 用 Rand7() 实现 Rand10()
已有方法 ```rand7``` 可生成 1 到 7 范围内的均匀随机整数，试写一个方法 ```rand10``` 生成 1 到 10 范围内的均匀随机整数。

不要使用系统的 ```Math.random()``` 方法。

#### 示例 1:
<pre>
<strong>输入:</strong> 1
<strong>输出:</strong> [7]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> [8,4]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> [8,1,10]
</pre>

#### 提示:
1. ```rand7``` 已定义。
2. 传入参数: ```n``` 表示 ```rand10``` 的调用次数。

#### 进阶:
1. ```rand7()```调用次数的 [期望值](https://en.wikipedia.org/wiki/Expected_value) 是多少 ?
2. 你能否尽量少调用 ```rand7()``` ?

## 题解 (Rust)

### 1. 题解
```Rust
/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut a = rand7();
        let mut b = rand7() * 7;

        while a + b > 47 {
            a = rand7();
            b = rand7() * 7;
        }

        (a + b + 2) % 10 + 1
    }
}
```
