# 1233. 删除子文件夹
你是一位系统管理员，手里有一份文件夹列表 `folder`，你的任务是要删除该列表中的所有 **子文件夹**，并以 **任意顺序** 返回剩下的文件夹。

我们这样定义「子文件夹」：
* 如果文件夹 `folder[i]` 位于另一个文件夹 `folder[j]` 下，那么 `folder[i]` 就是 `folder[j]` 的子文件夹。

文件夹的「路径」是由一个或多个按以下格式串联形成的字符串：
* `/` 后跟一个或者多个小写英文字母。

例如，`/leetcode` 和 `/leetcode/problems` 都是有效的路径，而空字符串和 `/` 不是。

#### 示例 1:
<pre>
<strong>输入:</strong> folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
<strong>输出:</strong> ["/a","/c/d","/c/f"]
<strong>解释:</strong> "/a/b/" 是 "/a" 的子文件夹，而 "/c/d/e" 是 "/c/d" 的子文件夹。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> folder = ["/a","/a/b/c","/a/b/d"]
<strong>输出:</strong> ["/a"]
<strong>解释:</strong> 文件夹 "/a/b/c" 和 "/a/b/d/" 都会被删除，因为它们都是 "/a" 的子文件夹。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> folder = ["/a/b/c","/a/b/ca","/a/b/d"]
<strong>输出:</strong> ["/a/b/c","/a/b/ca","/a/b/d"]
</pre>

#### 提示:
* `1 <= folder.length <= 4 * 10^4`
* `2 <= folder[i].length <= 100`
* `folder[i]` 只包含小写字母和 `/`
* `folder[i]` 总是以字符 `/` 起始
* 每个文件夹名都是唯一的

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder.into_iter().map(|s| s + "/").collect::<Vec<_>>();
        folder.sort_unstable();
        let mut ret = Vec::new();

        for f in folder {
            if ret.is_empty() || !f.starts_with(ret.last().unwrap()) {
                ret.push(f);
            }
        }
        ret.iter_mut().for_each(|s| {
            s.pop();
        });

        ret
    }
}
```
