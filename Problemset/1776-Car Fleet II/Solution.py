class Solution:
    def getCollisionTimes(self, cars: List[List[int]]) -> List[float]:
        stack = []
        answer = [-1] * len(cars)

        for i in range(len(cars) - 1, -1, -1):
            while stack and stack[-1][-1] >= cars[i][1]:
                stack.pop()

            while len(stack) > 1 and (stack[-1][0] - cars[i][0]) / (cars[i][1] - stack[-1][1]) > (stack[-2][0] - cars[i][0]) / (cars[i][1] - stack[-2][1]):
                stack.pop()

            if stack:
                answer[i] = (stack[-1][0] - cars[i][0]) / \
                    (cars[i][1] - stack[-1][1])
            stack.append(cars[i])

        return answer
