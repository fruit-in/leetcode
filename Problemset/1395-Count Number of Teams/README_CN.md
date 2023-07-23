# 1395. 统计作战单位数
```n``` 名士兵站成一排。每个士兵都有一个 **独一无二** 的评分 ```rating``` 。

每 **3** 个士兵可以组成一个作战单位，分组规则如下：
* 从队伍中选出下标分别为 ```i```、```j```、```k``` 的 3 名士兵，他们的评分分别为 ```rating[i]```、```rating[j]```、```rating[k]```
* 作战单位需满足： ```rating[i] < rating[j] < rating[k]``` 或者 ```rating[i] > rating[j] > rating[k]``` ，其中  ```0 <= i < j < k < n```

请你返回按上述条件可以组建的作战单位数量。每个士兵都可以是多个作战单位的一部分。

#### 示例 1:
<pre>
<strong>输入:</strong> rating = [2,5,3,4,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 我们可以组建三个作战单位 (2,3,4)、(5,4,1)、(5,3,1) 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> rating = [2,1,3]
<strong>输出:</strong> 0
<strong>解释:</strong> 根据题目条件，我们无法组建作战单位。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> rating = [1,2,3,4]
<strong>输出:</strong> 4
</pre>

#### 提示:
* ```n == rating.length```
* ```1 <= n <= 200```
* ```1 <= rating[i] <= 10^5```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut ret = 0;

        for j in 1..(rating.len() - 1) {
            let ltl = rating[..j].iter().filter(|&&x| x < rating[j]).count();
            let gtl = j - ltl;
            let ltr = rating[j..].iter().filter(|&&x| x < rating[j]).count();
            let gtr = rating.len() - 1 - j - ltr;

            ret += ltl * gtr + ltr * gtl;
        }

        ret as i32
    }
}
```
