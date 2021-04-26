# 433. 最小基因变化
一条基因序列由一个带有8个字符的字符串表示，其中每个字符都属于 `"A"`, `"C"`, `"G"`, `"T"`中的任意一个。

假设我们要调查一个基因序列的变化。一次基因变化意味着这个基因序列中的一个字符发生了变化。

例如，基因序列由`"AACCGGTT"` 变化至 `"AACCGGTA"` 即发生了一次基因变化。

与此同时，每一次基因变化的结果，都需要是一个合法的基因串，即该结果属于一个基因库。

现在给定3个参数 — start, end, bank，分别代表起始基因序列，目标基因序列及基因库，请找出能够使起始基因序列变化为目标基因序列所需的最少变化次数。如果无法实现目标变化，请返回 -1。

#### 注意:
1. 起始基因序列默认是合法的，但是它并不一定会出现在基因库中。
2. 如果一个起始基因序列需要多次变化，那么它每一次变化之后的基因序列都必须是合法的。
3. 假定起始基因序列与目标基因序列是不一样的。

#### 示例 1:
```
start: "AACCGGTT"
end:   "AACCGGTA"
bank: ["AACCGGTA"]

返回值: 1
```

#### 示例 2:
```
start: "AACCGGTT"
end:   "AAACGGTA"
bank: ["AACCGGTA", "AACCGCTA", "AAACGGTA"]

返回值: 2
```

#### 示例 3:
```
start: "AAAAACCC"
end:   "AACCCCCC"
bank: ["AAAACCCC", "AAACCCCC", "AACCCCCC"]

返回值: 3
```

## 题解 (Python)

### 1. 广度优先搜索
```Python
class Solution:
    def minMutation(self, start: str, end: str, bank: List[str]) -> int:
        seen = {start}
        genes = deque([(start, 0)])

        while len(genes) > 0:
            gene, n = genes.popleft()
            if gene == end:
                return n

            for i in range(8):
                for c in "ACGT":
                    new_gene = gene[:i] + c + gene[i + 1:]
                    if new_gene in bank and new_gene not in seen:
                        seen.add(new_gene)
                        genes.append((new_gene, n + 1))

        return -1
```

## 题解 (Ruby)

### 1. 广度优先搜索
```Ruby
# @param {String} start
# @param {String} end
# @param {String[]} bank
# @return {Integer}
def min_mutation(start, end_, bank)
  seen = Set[start]
  genes = Containers::Queue.new([[start, 0]])

  until genes.empty?
    gene, n = genes.pop
    return n if gene == end_

    (0..7).each do |i|
      'ACGT'.each_char do |c|
        new_gene = gene[0...i] + c + gene[i + 1..]
        if bank.member?(new_gene) && !seen.member?(new_gene)
          seen.add(new_gene)
          genes.push([new_gene, n + 1])
        end
      end
    end
  end

  -1
end
```
