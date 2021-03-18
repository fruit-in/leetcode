# 1233. Remove Sub-Folders from the Filesystem
Given a list of folders, remove all sub-folders in those folders and return in **any order** the folders after removing.

If a `folder[i]` is located within another `folder[j]`, it is called a sub-folder of it.

The format of a path is one or more concatenated strings of the form: `/` followed by one or more lowercase English letters. For example, `/leetcode` and `/leetcode/problems` are valid paths while an empty string and `/` are not.

#### Example 1:
<pre>
<strong>Input:</strong> folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
<strong>Output:</strong> ["/a","/c/d","/c/f"]
<strong>Explanation:</strong> Folders "/a/b/" is a subfolder of "/a" and "/c/d/e" is inside of folder "/c/d" in our filesystem.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> folder = ["/a","/a/b/c","/a/b/d"]
<strong>Output:</strong> ["/a"]
<strong>Explanation:</strong> Folders "/a/b/c" and "/a/b/d/" will be removed because they are subfolders of "/a".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> folder = ["/a/b/c","/a/b/ca","/a/b/d"]
<strong>Output:</strong> ["/a/b/c","/a/b/ca","/a/b/d"]
</pre>

#### Constraints:
* `1 <= folder.length <= 4 * 10^4`
* `2 <= folder[i].length <= 100`
* `folder[i]` contains only lowercase letters and '/'
* `folder[i]` always starts with character '/'
* Each folder name is unique.

## Solutions (Rust)

### 1. Sort
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
