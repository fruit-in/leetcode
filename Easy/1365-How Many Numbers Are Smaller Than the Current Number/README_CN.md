# 1365. 有多少小于当前数字的数字
给你一个数组 ```nums```，对于其中每个元素 ```nums[i]```，请你统计数组中比它小的所有数字的数目。

换而言之，对于每个 ```nums[i]``` 你必须计算出有效的 ```j``` 的数量，其中 ```j``` 满足 ```j != i``` **且** ```nums[j] < nums[i]``` 。

以数组形式返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [8,1,2,2,3]
<strong>输出:</strong> [4,0,1,1,3]
<strong>解释:</strong>
对于 nums[0]=8 存在四个比它小的数字：（1，2，2 和 3）。
对于 nums[1]=1 不存在比它小的数字。
对于 nums[2]=2 存在一个比它小的数字：（1）。
对于 nums[3]=2 存在一个比它小的数字：（1）。
对于 nums[4]=3 存在三个比它小的数字：（1，2 和 2）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [6,5,4,8]
<strong>输出:</strong> [2,1,0,3]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [7,7,7,7]
<strong>输出:</strong> [0,0,0,0]
</pre>

#### 提示:
* ```2 <= nums.length <= 500```
* ```0 <= nums[i] <= 100```

## 题解 (JavaScript)

### 1. 暴力
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

### 2. 排序
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

### 3. 前缀和
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
