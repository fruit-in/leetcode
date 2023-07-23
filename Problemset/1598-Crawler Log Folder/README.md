# 1598. Crawler Log Folder
The Leetcode file system keeps a log each time some user performs a *change folder* operation.

The operations are described below:
* `"../"` : Move to the parent folder of the current folder. (If you are already in the main folder, **remain in the same folder**).
* `"./"` : Remain in the same folder.
* `"x/"` : Move to the child folder named `x` (This folder is **guaranteed to always exist**).

You are given a list of strings `logs` where `logs[i]` is the operation performed by the user at the `i`<sup>`th`</sup> step.

The file system starts in the main folder, then the operations in `logs` are performed.

Return *the minimum number of operations needed to go back to the main folder after the change folder operations*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/09/sample_11_1957.png)
<pre>
<b>Input:</b> logs = ["d1/","d2/","../","d21/","./"]
<b>Output:</b> 2
<b>Explanation:</b> Use this change folder operation "../" 2 times and go back to the main folder.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/09/09/sample_22_1957.png)
<pre>
<b>Input:</b> logs = ["d1/","d2/","./","d3/","../","d31/"]
<b>Output:</b> 3
</pre>

#### Example 3:
<pre>
<b>Input:</b> logs = ["d1/","../","../","../"]
<b>Output:</b> 0
</pre>

#### Constraints:
* `1 <= logs.length <= 10`<sup>`3`</sup>
* `2 <= logs[i].length <= 10`
* `logs[i]` contains lowercase English letters, digits, `'.'`, and `'/'`.
* `logs[i]` follows the format described in the statement.
* Folder names consist of lowercase English letters and digits.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {String[]} logs
# @return {Integer}
def min_operations(logs)
    ret = 0

    for log in logs
        if log == "../" and ret > 0
            ret -= 1
        elsif log != "../" and log != "./"
            ret += 1
        end
    end

    return ret
end
```
