impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let mut arr2 = arr2;
        let mut carry = 0;
        let mut ret = vec![];

        while !arr1.is_empty() || !arr2.is_empty() || carry != 0 {
            let x = arr1.pop().unwrap_or(0) + arr2.pop().unwrap_or(0) + carry;
            carry = 1 - (x + 2) / 2;
            ret.push(x.rem_euclid(2));
        }

        while ret.len() > 1 && *ret.last().unwrap() == 0 {
            ret.pop();
        }

        ret.reverse();

        ret
    }
}
