# 1487. Making File Names Unique
Given an array of strings `names` of size `n`. You will create `n` folders in your file system **such that**, at the `ith` minute, you will create a folder with the name `names[i]`.

Since two files **cannot** have the same name, if you enter a folder name which is previously used, the system will have a suffix addition to its name in the form of `(k)`, where, `k` is the **smallest positive integer** such that the obtained name remains unique.

Return *an array of strings of length `n`* where `ans[i]` is the actual name the system will assign to the `ith` folder when you create it.

#### Example 1:
<pre>
<b>Input:</b> names = ["pes","fifa","gta","pes(2019)"]
<b>Output:</b> ["pes","fifa","gta","pes(2019)"]
<b>Explanation:</b> Let's see how the file system creates folder names:
"pes" --> not assigned before, remains "pes"
"fifa" --> not assigned before, remains "fifa"
"gta" --> not assigned before, remains "gta"
"pes(2019)" --> not assigned before, remains "pes(2019)"
</pre>

#### Example 2:
<pre>
<b>Input:</b> names = ["gta","gta(1)","gta","avalon"]
<b>Output:</b> ["gta","gta(1)","gta(2)","avalon"]
<b>Explanation:</b> Let's see how the file system creates folder names:
"gta" --> not assigned before, remains "gta"
"gta(1)" --> not assigned before, remains "gta(1)"
"gta" --> the name is reserved, system adds (k), since "gta(1)" is also reserved, systems put k = 2. it becomes "gta(2)"
"avalon" --> not assigned before, remains "avalon"
</pre>

#### Example 3:
<pre>
<b>Input:</b> names = ["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece"]
<b>Output:</b> ["onepiece","onepiece(1)","onepiece(2)","onepiece(3)","onepiece(4)"]
<b>Explanation:</b> When the last folder is created, the smallest positive valid k is 4, and it becomes "onepiece(4)".
</pre>

#### Example 4:
<pre>
<b>Input:</b> names = ["wano","wano","wano","wano"]
<b>Output:</b> ["wano","wano(1)","wano(2)","wano(3)"]
<b>Explanation:</b> Just increase the value of k each time you create folder "wano".
</pre>

#### Example 5:
<pre>
<b>Input:</b> names = ["kaido","kaido(1)","kaido","kaido(1)"]
<b>Output:</b> ["kaido","kaido(1)","kaido(2)","kaido(1)(1)"]
<b>Explanation:</b> Please note that system adds the suffix (k) to current name even it contained the same suffix before.
</pre>

#### Constraints:
* `1 <= names.length <= 5 * 10^4`
* `1 <= names[i].length <= 20`
* `names[i]` consists of lower case English letters, digits and/or round brackets.

## Solutions (Ruby)

### 1. HashMap
```Ruby
# @param {String[]} names
# @return {String[]}
def get_folder_names(names)
    h = Hash.new(0)

    for i in 0...names.length
        if h[names[i]] > 0
            while h.key?(names[i] + "(%d)" % h[names[i]])
                h[names[i]] += 1
            end
            names[i] += "(%d)" % h[names[i]]
        end
        h[names[i]] += 1
    end

    return names
end
```
