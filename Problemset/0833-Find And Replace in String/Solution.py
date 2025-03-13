class Solution:
    def findReplaceString(self, s: str, indices: List[int], sources: List[str], targets: List[str]) -> str:
        occurs = []
        subs = []

        for i in range(len(indices)):
            if s[indices[i]:indices[i] + len(sources[i])] == sources[i]:
                occurs.append(
                    (indices[i], indices[i] + len(sources[i]), targets[i]))

        occurs.sort()
        occurs.append((len(s), len(s), ""))
        subs.append(s[:occurs[0][0]])

        for i in range(len(occurs) - 1):
            subs.append(occurs[i][2])
            subs.append(s[occurs[i][1]:occurs[i + 1][0]])

        return ''.join(subs)
