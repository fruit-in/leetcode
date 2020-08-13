# 1146. Snapshot Array
Implement a SnapshotArray that supports the following interface:
* `SnapshotArray(int length)` initializes an array-like data structure with the given length.  **Initially, each element equals 0**.
* `void set(index, val)` sets the element at the given `index` to be equal to `val`.
* `int snap()` takes a snapshot of the array and returns the `snap_id`: the total number of times we called `snap()` minus `1`.
* `int get(index, snap_id)` returns the value at the given `index`, at the time we took the snapshot with the given `snap_id`

#### Example 1:
<pre>
<strong>Input:</strong> ["SnapshotArray","set","snap","set","get"]
[[3],[0,5],[],[0,6],[0,0]]
<strong>Output:</strong> [null,null,0,null,5]
<strong>Explanation:</strong>
SnapshotArray snapshotArr = new SnapshotArray(3); // set the length to be 3
snapshotArr.set(0,5);  // Set array[0] = 5
snapshotArr.snap();  // Take a snapshot, return snap_id = 0
snapshotArr.set(0,6);
snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5
</pre>

#### Constraints:
* `1 <= length <= 50000`
* At most `50000` calls will be made to `set`, `snap`, and `get`.
* `0 <= index < length`
* `0 <= snap_id < `(the total number of times we call `snap()`)
* `0 <= val <= 10^9`

## Solutions (Ruby)

### 1. Solution
```Ruby
class SnapshotArray

=begin
    :type length: Integer
=end
    def initialize(length)
        @arrays = Array.new(length) {Array[Array[0, 0]]}
        @snap_id = 0
    end


=begin
    :type index: Integer
    :type val: Integer
    :rtype: Void
=end
    def set(index, val)
        if @snap_id > @arrays[index][-1][0]
            @arrays[index].push(Array[@snap_id, val])
        else
            @arrays[index][-1][1] = val
        end
    end


=begin
    :rtype: Integer
=end
    def snap()
        @snap_id += 1

        return @snap_id - 1
    end


=begin
    :type index: Integer
    :type snap_id: Integer
    :rtype: Integer
=end
    def get(index, snap_id)
        l, r = 0, @arrays[index].length - 1

        while l <= r
            m = (l + r) / 2

            if @arrays[index][m][0] == snap_id
                return @arrays[index][m][1]
            elsif @arrays[index][m][0] < snap_id
                l = m + 1
            else
                r = m - 1
            end
        end

        return @arrays[index][r][1]
    end


end

# Your SnapshotArray object will be instantiated and called as such:
# obj = SnapshotArray.new(length)
# obj.set(index, val)
# param_2 = obj.snap()
# param_3 = obj.get(index, snap_id)
```

## Solutions (Rust)

### 1. Solution
```Rust
struct SnapshotArray {
    arrays: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {

    fn new(length: i32) -> Self {
        Self {
            arrays: vec![vec![(0, 0)]; length as usize],
            snap_id: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        if self.snap_id > self.arrays[index as usize].last().unwrap().0 {
            self.arrays[index as usize].push((self.snap_id, val));
        } else {
            self.arrays[index as usize].last_mut().unwrap().1 = val;
        }
    }

    fn snap(&mut self) -> i32 {
        self.snap_id += 1;

        self.snap_id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        match self.arrays[index as usize].binary_search_by_key(&snap_id, |&(a, b)| a) {
            Ok(i) => self.arrays[index as usize][i].1,
            Err(i) => self.arrays[index as usize][i - 1].1,
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */
```
