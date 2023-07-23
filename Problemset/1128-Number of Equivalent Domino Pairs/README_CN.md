# 1128. 等价多米诺骨牌对的数量
给你一个由一些多米诺骨牌组成的列表 ```dominoes```。

如果其中某一张多米诺骨牌可以通过旋转 ```0``` 度或 ```180``` 度得到另一张多米诺骨牌，我们就认为这两张牌是等价的。

形式上，```dominoes[i] = [a, b]``` 和 ```dominoes[j] = [c, d]``` 等价的前提是 ```a==c``` 且 ```b==d```，或是 ```a==d``` 且 ```b==c```。

在 ```0 <= i < j < dominoes.length``` 的前提下，找出满足 ```dominoes[i]``` 和 ```dominoes[j]``` 等价的骨牌对 ```(i, j)``` 的数量。

#### 示例:
<pre>
<strong>输入:</strong> dominoes = [[1,2],[2,1],[3,4],[5,6]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* ```1 <= dominoes.length <= 40000```
* ```1 <= dominoes[i][j] <= 9```

## 题解 (Rust)

### 1. 计数
```Rust
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut cnt = [0; 100];
        let mut ret = 0;
        for domino in dominoes {
            let tens = domino[0].min(domino[1]) as usize;
            let units = domino[0].max(domino[1]) as usize;
            ret += cnt[tens * 10 + units];
            cnt[tens * 10 + units] += 1;
        }
        ret
    }
}
```
