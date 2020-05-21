# 71. Simplify Path
Given an **absolute path** for a file (Unix-style), simplify it. Or in other words, convert it to the **canonical path**.

In a UNIX-style file system, a period `.` refers to the current directory. Furthermore, a double period `..` moves the directory up a level.

Note that the returned canonical path must always begin with a slash `/`, and there must be only a single slash `/` between two directory names. The last directory name (if it exists) **must not** end with a trailing `/`. Also, the canonical path must be the **shortest** string representing the absolute path.

#### Example 1:
<pre>
<strong>Input:</strong> "/home/"
<strong>Output:</strong> "/home"
<strong>Explanation:</strong> Note that there is no trailing slash after the last directory name.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "/../"
<strong>Output:</strong> "/"
<strong>Explanation:</strong> Going one level up from the root directory is a no-op, as the root level is the highest level you can go.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "/home//foo/"
<strong>Output:</strong> "/home/foo"
<strong>Explanation:</strong> In the canonical path, multiple consecutive slashes are replaced by a single one.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> "/a/./b/../../c/"
<strong>Output:</strong> "/c"
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> "/a/../../b/../c//.//"
<strong>Output:</strong> "/c"
</pre>

#### Example 6:
<pre>
<strong>Input:</strong> "/a//b////c/d//././/.."
<strong>Output:</strong> "/a/b/c"
</pre>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def simplifyPath(self, path: str) -> str:
        stack = []

        for d in path.split('/'):
            if d == '..':
                if stack:
                    stack.pop()
            elif d != '.' and d != '':
                stack.append(d)

        return '/' + '/'.join(stack)
```
