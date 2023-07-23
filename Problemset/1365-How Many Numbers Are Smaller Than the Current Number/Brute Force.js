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
