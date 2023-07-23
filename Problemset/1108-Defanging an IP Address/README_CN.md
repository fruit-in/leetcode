# 1108. IP 地址无效化
给你一个有效的 [IPv4](https://baike.baidu.com/item/IPv4) 地址 ```address```，返回这个 IP 地址的无效化版本。

所谓无效化 IP 地址，其实就是用 ```"[.]"``` 代替了每个 ```"."```。

#### 示例 1:
<pre>
<strong>输入:</strong> address = "1.1.1.1"
<strong>输出:</strong> "1[.]1[.]1[.]1"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> address = "255.100.50.0"
<strong>输出:</strong> "255[.]100[.]50[.]0"
</pre>

#### 提示:
* 给出的 ```address``` 是一个有效的 IPv4 地址

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}
```
