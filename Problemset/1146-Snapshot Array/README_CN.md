# 1146. 快照数组
实现支持下列接口的「快照数组」- SnapshotArray：
* `SnapshotArray(int length)` - 初始化一个与指定长度相等的 类数组 的数据结构。**初始时，每个元素都等于 0**。
* `void set(index, val)` - 会将指定索引 `index` 处的元素设置为 `val`。
* `int snap()` - 获取该数组的快照，并返回快照的编号 `snap_id`（快照号是调用 `snap()` 的总次数减去 `1`）。
* `int get(index, snap_id)` - 根据指定的 `snap_id` 选择快照，并返回该快照指定索引 `index` 的值。

#### 示例:
<pre>
<strong>输入:</strong> ["SnapshotArray","set","snap","set","get"]
[[3],[0,5],[],[0,6],[0,0]]
<strong>输出:</strong> [null,null,0,null,5]
<strong>解释:</strong>
SnapshotArray snapshotArr = new SnapshotArray(3); // 初始化一个长度为 3 的快照数组
snapshotArr.set(0,5);  // 令 array[0] = 5
snapshotArr.snap();  // 获取快照，返回 snap_id = 0
snapshotArr.set(0,6);
snapshotArr.get(0,0);  // 获取 snap_id = 0 的快照中 array[0] 的值，返回 5
</pre>

#### 提示:
* `1 <= length <= 50000`
* 题目最多进行`50000` 次`set`，`snap`，和 `get`的调用 。
* `0 <= index < length`
* `0 <= snap_id < `我们调用 `snap()` 的总次数
* `0 <= val <= 10^9`

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
