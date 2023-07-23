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
