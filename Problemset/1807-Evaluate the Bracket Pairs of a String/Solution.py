class Solution:
    def evaluate(self, s: str, knowledge: List[List[str]]) -> str:
        knowledge = dict(knowledge)
        key = ""
        ret = ""

        for ch in s:
            if ch == ')':
                ret += knowledge.get(key[1:], "?")
                key = ""
            elif key != "" or ch == '(':
                key += ch
            else:
                ret += ch

        return ret
