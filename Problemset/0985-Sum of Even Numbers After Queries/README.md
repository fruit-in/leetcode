# 985. Sum of Even Numbers After Queries
We have an array <code>A</code> of integers, and an array <code>queries</code> of queries.

For the <code>i</code>-th query <code>val = queries[i][0], index = queries[i][1]</code>, we add val to <code>A[index]</code>. Then, the answer to the <code>i</code>-th query is the sum of the even values of <code>A</code>.

*(Here, the given <code>index = queries[i][1]</code> is a 0-based index, and each query permanently modifies the array <code>A</code>.)*

Return the <code>answer</code> to all queries. Your answer array should have <code>answer[i]</code> as the answer to the <code>i</code>-th query.

#### Example 1:
<pre>
<strong>Input:</strong> A = [1,2,3,4], queries = [[1,0],[-3,1],[-4,0],[2,3]]
<strong>Output:</strong> [8,6,2,4]
<strong>Explanation:</strong>
At the beginning, the array is [1,2,3,4].
After adding 1 to A[0], the array is [2,2,3,4], and the sum of even values is 2 + 2 + 4 = 8.
After adding -3 to A[1], the array is [2,-1,3,4], and the sum of even values is 2 + 4 = 6.
After adding -4 to A[0], the array is [-2,-1,3,4], and the sum of even values is -2 + 4 = 2.
After adding 2 to A[3], the array is [-2,-1,3,6], and the sum of even values is -2 + 6 = 4.
</pre>

#### Note:
1. <code>1 <= A.length <= 10000</code>
2. <code>-10000 <= A[i] <= 10000</code>
3. <code>1 <= queries.length <= 10000</code>
4. <code>-10000 <= queries[i][0] <= 10000</code>
5. <code>0 <= queries[i][1] < A.length</code>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def sumEvenAfterQueries(self, A: List[int], queries: List[List[int]]) -> List[int]:
        result = []
        S = sum(n for n in A if n % 2 == 0)
        for val, index in queries:
            if A[index] % 2 == 0:
                S -= A[index]
            A[index] += val
            if A[index] % 2 == 0:
                S += A[index]
            result.append(S)
        return result
```
