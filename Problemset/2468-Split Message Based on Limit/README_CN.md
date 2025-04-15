# 2468. 根据限制分割消息
给你一个字符串 `message` 和一个正整数 `limit` 。

你需要根据 `limit` 将 `message` **分割** 成一个或多个 **部分** 。每个部分的结尾都是 `"<a/b>"` ，其中 `"b"` 用分割出来的总数 **替换**， `"a"` 用当前部分所在的编号 **替换** ，编号从 `1` 到 `b` 依次编号。除此以外，除了最后一部分长度 **小于等于** `limit` 以外，其他每一部分（包括结尾部分）的长度都应该 **等于** `limit` 。

你需要确保分割后的结果数组，删掉每部分的结尾并 **按顺序** 连起来后，能够得到 `message` 。同时，结果数组越短越好。

请你返回 `message`  分割后得到的结果数组。如果无法按要求分割 `message` ，返回一个空数组。

#### 示例 1:
<pre>
<strong>输入:</strong> message = "this is really a very awesome message", limit = 9
<strong>输出:</strong> ["thi<1/14>","s i<2/14>","s r<3/14>","eal<4/14>","ly <5/14>","a v<6/14>","ery<7/14>"," aw<8/14>","eso<9/14>","me<10/14>"," m<11/14>","es<12/14>","sa<13/14>","ge<14/14>"]
<strong>解释:</strong>
前面 9 个部分分别从 message 中得到 3 个字符。
接下来的 5 个部分分别从 message 中得到 2 个字符。
这个例子中，包含最后一个部分在内，每个部分的长度都为 9 。
可以证明没有办法分割成少于 14 个部分。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> message = "short message", limit = 15
<strong>输出:</strong> ["short mess<1/2>","age<2/2>"]
<strong>解释:</strong>
在给定限制下，字符串可以分成两个部分：
- 第一个部分包含 10 个字符，长度为 15 。
- 第二个部分包含 3 个字符，长度为 8 。
</pre>

#### 提示:
* <code>1 <= message.length <= 10<sup>4</sup></code>
* `message` 只包含小写英文字母和 `' '` 。
* <code>1 <= limit <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    fn splitn(n: usize, mut length: usize, limit: usize) -> Option<usize> {
        let ceil = 10_usize.pow(n as u32);
        let max_index_size = 3 + n * 2;

        if limit <= max_index_size {
            return None;
        }

        length += (3 + n) * (ceil / 10 - 1);
        length += (1..n)
            .map(|x| x * 9 * 10_usize.pow(x as u32 - 1))
            .sum::<usize>();
        length = length.saturating_sub(limit * (ceil / 10 - 1));

        if length == 0
            || (length + limit - max_index_size - 1) / (limit - max_index_size) + ceil / 10 - 1
                >= ceil
        {
            return None;
        }

        Some((length + limit - max_index_size - 1) / (limit - max_index_size) + ceil / 10 - 1)
    }

    pub fn split_message(message: String, limit: i32) -> Vec<String> {
        let limit = limit as usize;
        let length = message.len();
        let mut b = 0;
        let mut start = 0;
        let mut ret = vec![];

        for n in 1..5 {
            if let Some(x) = Self::splitn(n, length, limit) {
                b = x;
                break;
            }
        }

        for a in 1..=b {
            let index = format!("<{}/{}>", a, b);
            let end = length.min(start + limit - index.len());

            ret.push(format!(
                "{}{}",
                message.get(start..end).unwrap_or(""),
                index
            ));
            start = end;
        }

        ret
    }
}
```
