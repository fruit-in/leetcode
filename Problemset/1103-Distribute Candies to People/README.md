# 1103. Distribute Candies to People
We distribute some number of ```candies```, to a row of **```n = num_people```** people in the following way:

We then give 1 candy to the first person, 2 candies to the second person, and so on until we give ```n``` candies to the last person.

Then, we go back to the start of the row, giving ```n + 1``` candies to the first person, ```n + 2``` candies to the second person, and so on until we give ```2 * n``` candies to the last person.

This process repeats (with us giving one more candy each time, and moving to the start of the row after we reach the end) until we run out of candies.  The last person will receive all of our remaining candies (not necessarily one more than the previous gift).

Return an array (of length ```num_people``` and sum ```candies```) that represents the final distribution of candies.

#### Example 1:
<pre>
<strong>Input:</strong> candies = 7, num_people = 4
<strong>Output:</strong> [1,2,3,1]
<strong>Explanation:</strong>
On the first turn, ans[0] += 1, and the array is [1,0,0,0].
On the second turn, ans[1] += 2, and the array is [1,2,0,0].
On the third turn, ans[2] += 3, and the array is [1,2,3,0].
On the fourth turn, ans[3] += 1 (because there is only one candy left), and the final array is [1,2,3,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> candies = 10, num_people = 3
<strong>Output:</strong> [5,2,3]
<strong>Explanation:</strong>
On the first turn, ans[0] += 1, and the array is [1,0,0].
On the second turn, ans[1] += 2, and the array is [1,2,0].
On the third turn, ans[2] += 3, and the array is [1,2,3].
On the fourth turn, ans[0] += 4, and the final array is [5,2,3].
</pre>

#### Constraints:
* 1 <= candies <= 10^9
* 1 <= num_people <= 1000

## Solutions (Python)

### 1. Simulation
```Python3
class Solution:
    def distributeCandies(self, candies: int, num_people: int) -> List[int]:
        ret = [0] * num_people
        give = 1
        i = 0

        while candies:
            ret[i] += min(give, candies)
            candies -= min(give, candies)
            give += 1
            i = (i + 1) % num_people

        return ret
```

### 2. Mathematical
```Python3
class Solution:
    def distributeCandies(self, candies: int, num_people: int) -> List[int]:
        m = math.ceil(((1 + 8 * candies) ** .5 - 1) / 2)
        y = (m - 1) // num_people
        x = (m - 1) % num_people

        ret = []
        for i in range(num_people):
            if i < x:
                ret.append(y * (y + 1) * num_people // 2 + (y + 1) * (i + 1))
            elif i > x:
                ret.append(y * (y - 1) * num_people // 2 + y * (i + 1))
            else:
                ret.append(y * (y - 1) * num_people // 2 + y * (i + 1) + candies - m * (m - 1) // 2)

        return ret
```
