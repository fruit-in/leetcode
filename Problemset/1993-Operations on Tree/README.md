# 1993. Operations on Tree
You are given a tree with `n` nodes numbered from `0` to `n - 1` in the form of a parent array `parent` where `parent[i]` is the parent of the <code>i<sup>th</sup></code> node. The root of the tree is node `0`, so `parent[0] = -1` since it has no parent. You want to design a data structure that allows users to lock, unlock, and upgrade nodes in the tree.

The data structure should support the following functions:

* **Lock: Locks** the given node for the given user and prevents other users from locking the same node. You may only lock a node using this function if the node is unlocked.
* **Unlock: Unlocks** the given node for the given user. You may only unlock a node using this function if it is currently locked by the same user.
* **Upgrade: Locks** the given node for the given user and **unlocks** all of its descendants **regardless** of who locked it. You may only upgrade a node if **all** 3 conditions are true:
    * The node is unlocked,
    * It has at least one locked descendant (by **any** user), and
    * It does not have any locked ancestors.

Implement the `LockingTree` class:

* `LockingTree(int[] parent)` initializes the data structure with the parent array.
* `lock(int num, int user)` returns `true` if it is possible for the user with id `user` to lock the node `num`, or `false` otherwise. If it is possible, the node `num` will become **locked** by the user with id `user`.
* `unlock(int num, int user)` returns `true` if it is possible for the user with id `user` to unlock the node `num`, or `false` otherwise. If it is possible, the node `num` will become **unlocked**.
* `upgrade(int num, int user)` returns `true` if it is possible for the user with id `user` to upgrade the node `num`, or `false` otherwise. If it is possible, the node `num` will be **upgraded**.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/29/untitled.png)
<pre>
<strong>Input:</strong>
["LockingTree", "lock", "unlock", "unlock", "lock", "upgrade", "lock"]
[[[-1, 0, 0, 1, 1, 2, 2]], [2, 2], [2, 3], [2, 2], [4, 5], [0, 1], [0, 1]]
<strong>Output:</strong>
[null, true, false, true, true, true, false]
<strong>Explanation:</strong>
LockingTree lockingTree = new LockingTree([-1, 0, 0, 1, 1, 2, 2]);
lockingTree.lock(2, 2);    // return true because node 2 is unlocked.
                           // Node 2 will now be locked by user 2.
lockingTree.unlock(2, 3);  // return false because user 3 cannot unlock a node locked by user 2.
lockingTree.unlock(2, 2);  // return true because node 2 was previously locked by user 2.
                           // Node 2 will now be unlocked.
lockingTree.lock(4, 5);    // return true because node 4 is unlocked.
                           // Node 4 will now be locked by user 5.
lockingTree.upgrade(0, 1); // return true because node 0 is unlocked and has at least one locked descendant (node 4).
                           // Node 0 will now be locked by user 1 and node 4 will now be unlocked.
lockingTree.lock(0, 1);    // return false because node 0 is already locked.
</pre>

#### Constraints:
* `n == parent.length`
* `2 <= n <= 2000`
* `0 <= parent[i] <= n - 1` for `i != 0`
* `parent[0] == -1`
* `0 <= num <= n - 1`
* <code>1 <= user <= 10<sup>4</sup></code>
* `parent` represents a valid tree.
* At most `2000` calls **in total** will be made to `lock`, `unlock`, and `upgrade`.

## Solutions (Rust)

### 1. Solution
```Rust
struct LockingTree {
    parent: Vec<i32>,
    children: Vec<Vec<i32>>,
    locked: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let n = parent.len();
        let mut children = vec![vec![]; n];

        for i in 1..n {
            children[parent[i] as usize].push(i as i32);
        }

        Self {
            parent: parent,
            children: children,
            locked: vec![-1; n],
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] == -1 {
            self.locked[num as usize] = user;

            true
        } else {
            false
        }
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        if self.locked[num as usize] == user {
            self.locked[num as usize] = -1;

            true
        } else {
            false
        }
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let mut curr = num;
        let mut nodes = self.children[num as usize].clone();
        let mut locked_descendants = vec![];

        while curr != -1 {
            if self.locked[curr as usize] != -1 {
                return false;
            }

            curr = self.parent[curr as usize];
        }

        while let Some(node) = nodes.pop() {
            nodes.append(&mut self.children[node as usize].clone());

            if self.locked[node as usize] != -1 {
                locked_descendants.push(node as usize);
            }
        }

        if !locked_descendants.is_empty() {
            self.lock(num, user);
            for node in locked_descendants {
                self.locked[node] = -1;
            }

            true
        } else {
            false
        }
    }
}

/**
 * Your LockingTree object will be instantiated and called as such:
 * let obj = LockingTree::new(parent);
 * let ret_1: bool = obj.lock(num, user);
 * let ret_2: bool = obj.unlock(num, user);
 * let ret_3: bool = obj.upgrade(num, user);
 */
```
