# 609. 在系统中查找重复文件
给定一个目录信息列表，包括目录路径，以及该目录中的所有包含内容的文件，您需要找到文件系统中的所有重复文件组的路径。一组重复的文件至少包括二个具有完全相同内容的文件。

**输入**列表中的单个目录信息字符串的格式如下：
```
"root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"
```

这意味着有 n 个文件（`f1.txt`, `f2.txt` ... `fn.txt` 的内容分别是 `f1_content`, `f2_content` ... `fn_content`）在目录 `root/d1/d2/.../dm` 下。注意：n>=1 且 m>=0。如果 m=0，则表示该目录是根目录。

该**输出**是重复文件路径组的列表。对于每个组，它包含具有相同内容的文件的所有文件路径。文件路径是具有下列格式的字符串：
```
"directory_path/file_name.txt"
```

#### 示例 1:
<pre>
<b>输入:</b>
["root/a 1.txt(abcd) 2.txt(efgh)", "root/c 3.txt(abcd)", "root/c/d 4.txt(efgh)", "root 4.txt(efgh)"]
<b>输出:</b>
[["root/a/2.txt","root/c/d/4.txt","root/4.txt"],["root/a/1.txt","root/c/3.txt"]]
</pre>

#### 注:
1. 最终输出不需要顺序。
2. 您可以假设目录名、文件名和文件内容只有字母和数字，并且文件内容的长度在 [1，50] 的范围内。
3. 给定的文件数量在 [1，20000] 个范围内。
4. 您可以假设在同一目录中没有任何文件或目录共享相同的名称。
5. 您可以假设每个给定的目录信息代表一个唯一的目录。目录路径和文件信息用一个空格分隔。

#### 超越竞赛的后续行动:
1. 假设您有一个真正的文件系统，您将如何搜索文件？广度搜索还是宽度搜索？
2. 如果文件内容非常大（GB级别），您将如何修改您的解决方案？
3. 如果每次只能读取 1 kb 的文件，您将如何修改解决方案？
4. 修改后的解决方案的时间复杂度是多少？其中最耗时的部分和消耗内存的部分是什么？如何优化？
5. 如何确保您发现的重复文件不是误报？

## 题解 (Ruby)

### 1. 哈希表
```Ruby
# @param {String[]} paths
# @return {String[][]}
def find_duplicate(paths)
    h = Hash.new {|hash, key| hash[key] = []}

    for path in paths
        path = path.split(' ')

        for file in path[1..]
            i = file.index('(')

            file_path = path[0] + '/' + file[0...i]
            content = file[(i + 1)...-1]

            h[content].push(file_path)
        end
    end

    return h.values.select {|elem| elem.length > 1}
end
```
