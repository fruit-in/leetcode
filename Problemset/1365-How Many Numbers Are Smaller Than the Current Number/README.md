# 1365. How Many Numbers Are Smaller Than the Current Number
Given the array ```nums```, for each ```nums[i]``` find out how many numbers in the array are smaller than it. That is, for each ```nums[i]``` you have to count the number of valid ```j's``` such that ```j != i``` **and** ```nums[j] < nums[i]```.

Return the answer in an array.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [8,1,2,2,3]
<strong>Output:</strong> [4,0,1,1,3]
<strong>Explanation:</strong>
For nums[0]=8 there exist four smaller numbers than it (1, 2, 2 and 3).
For nums[1]=1 does not exist any smaller number than it.
For nums[2]=2 there exist one smaller number than it (1).
For nums[3]=2 there exist one smaller number than it (1).
For nums[4]=3 there exist three smaller numbers than it (1, 2 and 2).
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [6,5,4,8]
<strong>Output:</strong> [2,1,0,3]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [7,7,7,7]
<strong>Output:</strong> [0,0,0,0]
</pre>

#### Constraints:
* ```2 <= nums.length <= 500```
* ```0 <= nums[i] <= 100```

## Solutions (JavaScript)

### 1. Brute Force
```JavaScript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var smallerNumbersThanCurrent = function(nums) {
    let ret = [];

    for(let i = 0, cnt = 0; i < nums.length; i++, cnt = 0) {
        for(let j = 0; j < nums.length; j++) {
            if(nums[j] < nums[i]) {
                cnt++;
            }
        }
        ret.push(cnt);
    }

    return ret;
};
```

### 2. Sort
```JavaScript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var smallerNumbersThanCurrent = function(nums) {
    const sorted = nums.slice().sort((a, b) => a - b);
    return nums.map(num => sorted.indexOf(num));
};
```

### 3. Prefix Sum
```JavaScript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var smallerNumbersThanCurrent = function(nums) {
    let prefix = new Array(101).fill(0);
    nums.forEach(num => prefix[num]++);

    for(let i = 1; i < 101; i++) {
        prefix[i] += prefix[i - 1];
    }

    return nums.map(num => num > 0 ? prefix[num - 1] : 0);
};
```
