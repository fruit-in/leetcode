class Solution:
    def numSmallerByFrequency(self, queries: List[str], words: List[str]) -> List[int]:
        queries_cnt = [query.count(min(query)) for query in queries]
        words_cnt = [word.count(min(word)) for word in words]
        ret = []

        for q_cnt in queries_cnt:
            ret.append(sum(1 for w_cnt in words_cnt if w_cnt > q_cnt))

        return ret
