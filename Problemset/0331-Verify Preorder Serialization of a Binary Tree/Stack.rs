impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack = vec![];

        for x in preorder.split(',') {
            stack.push(x);
            while let Some(&[y, "#", "#"]) = stack.get(stack.len() - 3..stack.len()) {
                if y == "#" {
                    break;
                } else {
                    stack.pop();
                    stack.pop();
                    stack.pop();
                    stack.push("#");
                }
            }
        }

        &stack == &["#"]
    }
}
