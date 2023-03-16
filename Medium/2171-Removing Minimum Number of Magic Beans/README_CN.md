# 2171. 拿出最少数目的魔法豆
给你一个 **正** 整数数组 `beans` ，其中每个整数表示一个袋子里装的魔法豆的数目。

请你从每个袋子中 **拿出** 一些豆子（也可以 **不拿出**），使得剩下的 **非空** 袋子中（即 **至少** 还有 **一颗** 魔法豆的袋子）魔法豆的数目 **相等** 。一旦魔法豆从袋子中取出，你不能将它放到任何其他的袋子中。

请你返回你需要拿出魔法豆的 **最少数目**。

#### 示例 1:
<pre>
<strong>输入:</strong> beans = [4,1,6,5]
<strong>输出:</strong> 4
<strong>解释:</strong>
- 我们从有 1 个魔法豆的袋子中拿出 1 颗魔法豆。
  剩下袋子中魔法豆的数目为：[4,0,6,5]
- 然后我们从有 6 个魔法豆的袋子中拿出 2 个魔法豆。
  剩下袋子中魔法豆的数目为：[4,0,4,5]
- 然后我们从有 5 个魔法豆的袋子中拿出 1 个魔法豆。
  剩下袋子中魔法豆的数目为：[4,0,4,4]
总共拿出了 1 + 2 + 1 = 4 个魔法豆，剩下非空袋子中魔法豆的数目相等。
没有比取出 4 个魔法豆更少的方案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> beans = [2,10,3,2]
<strong>输出:</strong> 7
<strong>解释:</strong>
- 我们从有 2 个魔法豆的其中一个袋子中拿出 2 个魔法豆。
  剩下袋子中魔法豆的数目为：[0,10,3,2]
- 然后我们从另一个有 2 个魔法豆的袋子中拿出 2 个魔法豆。
  剩下袋子中魔法豆的数目为：[0,10,3,0]
- 然后我们从有 3 个魔法豆的袋子中拿出 3 个魔法豆。
  剩下袋子中魔法豆的数目为：[0,10,0,0]
总共拿出了 2 + 2 + 3 = 7 个魔法豆，剩下非空袋子中魔法豆的数目相等。
没有比取出 7 个魔法豆更少的方案。
</pre>

#### 提示:
* <code>1 <= beans.length <= 10<sup>5</sup></code>
* <code>1 <= beans[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_removal(beans: Vec<i32>) -> i64 {
        let mut beans = beans.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mut lsum = 0;
        let mut rsum = beans.iter().sum::<i64>();
        let mut ret = i64::MAX;
        beans.sort_unstable();

        for i in 0..beans.len() {
            ret = ret.min(lsum + rsum - (beans.len() - i) as i64 * beans[i]);
            lsum += beans[i];
            rsum -= beans[i];
        }

        ret
    }
}
```
