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
