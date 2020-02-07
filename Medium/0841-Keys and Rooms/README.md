# 841. Keys and Rooms
There are ```N``` rooms and you start in room ```0```.  Each room has a distinct number in ```0, 1, 2, ..., N-1```, and each room may have some keys to access the next room.

Formally, each room ```i``` has a list of keys ```rooms[i]```, and each key ```rooms[i][j]``` is an integer in ```[0, 1, ..., N-1]``` where ```N = rooms.length```.  A key ```rooms[i][j] = v``` opens the room with number ```v```.

Initially, all the rooms start locked (except for room ```0```).

You can walk back and forth between rooms freely.

Return ```true``` if and only if you can enter every room.

#### Example 1:
<pre>
<strong>Input:</strong> [[1],[2],[3],[]]
<strong>Output:</strong> true
<strong>Explanation:</strong>
We start in room 0, and pick up key 1.
We then go to room 1, and pick up key 2.
We then go to room 2, and pick up key 3.
We then go to room 3.  Since we were able to go to every room, we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,3],[3,0,1],[2],[0]]
<strong>Output:</strong> false
<strong>Explanation:</strong> We can't enter the room with number 2.
</pre>

#### Note:
1. ```1 <= rooms.length <= 1000```
2. ```0 <= rooms[i].length <= 1000```
3. The number of keys in all rooms combined is at most ```3000```.

## Solutions (Rust)

### 1. DFS
```Rust
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut opened = vec![false; rooms.len()];
        let mut keys = vec![0];

        while let Some(curr) = keys.pop() {
            if !opened[curr] {
                opened[curr] = true;
                for &key in &rooms[curr] {
                    if !opened[key as usize] {
                        keys.push(key as usize);
                    }
                }
            }
        }

        opened.iter().all(|&room| room)
    }
}
```
