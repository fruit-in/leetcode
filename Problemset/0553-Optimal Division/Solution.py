class Solution:
    def optimalDivision(self, nums: List[int]) -> str:
        expression = "/".join(map(str, nums[1:]))
        if len(nums) > 2:
            expression = "(" + expression + ")"
        if len(nums) > 1:
            expression = "/" + expression
        expression = str(nums[0]) + expression

        return expression
