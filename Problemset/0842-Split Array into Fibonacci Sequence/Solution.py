class Solution:
    def splitIntoFibonacci(self, num: str) -> List[int]:
        for i in range(1, min(11, (len(num) + 1) // 2)):
            if i > 1 and num[0] == '0':
                break

            for j in range(i + 1, min(i + 11, len(num))):
                if j - i > 1 and num[i] == '0':
                    break

                seq = [int(num[:i]), int(num[i:j])]
                k = j

                while k < len(num):
                    size = len(str(seq[-2] + seq[-1]))
                    seq.append(int(num[k:k + size]))
                    k += size

                    if seq[-1] >= 2147483648 or seq[-3] + seq[-2] != seq[-1]:
                        seq = []
                        break

                if seq != []:
                    return seq

        return []
