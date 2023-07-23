# 1389. Create Target Array in the Given Order
Given two arrays of integers ```nums``` and ```index```. Your task is to create *target* array under the following rules:
* Initially *target* array is empty.
* From left to right read nums[i] and index[i], insert at index ```index[i]``` the value ```nums[i]``` in *target* array.
* Repeat the previous step until there are no elements to read in ```nums``` and ```index```.

Return the *target* array.

It is guaranteed that the insertion operations will be valid.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,1,2,3,4], index = [0,1,2,2,1]
<strong>Output:</strong> [0,4,1,3,2]
<strong>Explanation:</strong>
nums       index     target
0            0        [0]
1            1        [0,1]
2            2        [0,1,2]
3            2        [0,1,3,2]
4            1        [0,4,1,3,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,0], index = [0,1,2,3,0]
<strong>Output:</strong> [0,1,2,3,4]
<strong>Explanation:</strong>
nums       index     target
1            0        [1]
2            1        [1,2]
3            2        [1,2,3]
4            3        [1,2,3,4]
0            0        [0,1,2,3,4]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1], index = [0]
<strong>Output:</strong> [1]
</pre>

#### Constraints:
* ```1 <= nums.length, index.length <= 100```
* ```nums.length == index.length```
* ```0 <= nums[i] <= 100```
* ```0 <= index[i] <= i```

## Solutions (Ruby)

### 1. Simulation
```Ruby
# @param {Integer[]} nums
# @param {Integer[]} index
# @return {Integer[]}
def create_target_array(nums, index)
    target = Array.new

    for i in 0...nums.length
        target.insert(index[i], nums[i])
    end

    return target
end
```

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut target = Vec::new();

        for i in 0..nums.len() {
            target.insert(index[i] as usize, nums[i]);
        }

        target
    }
}
```
