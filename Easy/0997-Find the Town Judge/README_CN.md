# 997. 找到小镇的法官
在一个小镇里，按从 ```1``` 到 ```N``` 标记了 ```N``` 个人。传言称，这些人中有一个是小镇上的秘密法官。

如果小镇的法官真的存在，那么：
1. 小镇的法官不相信任何人。
2. 每个人（除了小镇法官外）都信任小镇的法官。
3. 只有一个人同时满足属性 1 和属性 2 。

给定数组 ```trust```，该数组由信任对 ```trust[i] = [a, b]``` 组成，表示标记为 ```a``` 的人信任标记为 ```b``` 的人。

如果小镇存在秘密法官并且可以确定他的身份，请返回该法官的标记。否则，返回 ```-1```。

#### 示例 1:
<pre>
<strong>输入:</strong> N = 2, trust = [[1,2]]
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> N = 3, trust = [[1,3],[2,3]]
<strong>输出:</strong> 3
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> N = 3, trust = [[1,3],[2,3],[3,1]]
<strong>输出:</strong> -1
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> N = 3, trust = [[1,2],[2,3]]
<strong>输出:</strong> -1
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> N = 4, trust = [[1,3],[1,4],[2,3],[2,4],[4,3]]
<strong>输出:</strong> 3
</pre>

#### 提示:
1. ```1 <= N <= 1000```
2. ```trust.length <= 10000```
3. ```trust[i]``` 是完全不同的
4. ```trust[i][0] != trust[i][1]```
5. ```1 <= trust[i][0], trust[i][1] <= N```

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut people = vec![0; n as usize];

        for t in trust {
            people[t[0] as usize - 1] -= 1;
            people[t[1] as usize - 1] += 1;
        }

        for i in 0..n {
            if people[i as usize] == n - 1 {
                return i + 1;
            }
        }

        -1
    }
}
```
