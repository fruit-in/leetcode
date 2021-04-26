# 433. Minimum Genetic Mutation
A gene string can be represented by an 8-character long string, with choices from `'A'`, `'C'`, `'G'`, and `'T'`.

Suppose we need to investigate a mutation from a gene string `start` to a gene string `end` where one mutation is defined as one single character changed in the gene string.
* For example, `"AACCGGTT" --> "AACCGGTA"` is one mutation.

There is also a gene bank `bank` that records all the valid gene mutations. A gene must be in `bank` to make it a valid gene string.

Given the two gene strings `start` and `end` and the gene bank `bank`, return *the minimum number of mutations needed to mutate from* `start` *to* `end`. If there is no such a mutation, return `-1`.

Note that the starting point is assumed to be valid, so it might not be included in the bank.

#### Example 1:
<pre>
<strong>Input:</strong> start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","AACCCCCC"]
<strong>Output:</strong> 3
</pre>

#### Constraints:
* `start.length == 8`
* `end.length == 8`
* `0 <= bank.length <= 10`
* `bank[i].length == 8`
* `start`, `end`, and `bank[i]` consist of only the characters `['A', 'C', 'G', 'T']`.

## Solutions (Python)

### 1. BFS
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

## Solutions (Ruby)

### 1. BFS
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
