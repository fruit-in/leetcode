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
