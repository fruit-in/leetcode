impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = n.to_string().into_bytes();
        let mut indices = vec![];
        let mut exist = false;
        let mut ret: i32 = 0;

        for i in (0..digits.len()).rev() {
            match indices.binary_search_by_key(&digits[i], |&j| digits[j]) {
                Ok(k) if k != indices.len() - 1 => {
                    digits.swap(i, indices[k + 1]);
                    exist = true;
                }
                Ok(k) => (),
                Err(k) if k != indices.len() => {
                    digits.swap(i, indices[k]);
                    exist = true;
                }
                Err(k) => indices.push(i),
            }

            if exist {
                let mut tmp = digits.split_off(i + 1);
                tmp.sort_unstable();
                digits.append(&mut tmp);
                break;
            }
        }

        for i in 0..digits.len() {
            let (tmp, overflow) = ret.overflowing_mul(10);
            ret = tmp;
            exist &= !overflow;
            let (tmp, overflow) = ret.overflowing_add((digits[i] - b'0') as i32);
            ret = tmp;
            exist &= !overflow;

            if !exist {
                return -1;
            }
        }

        ret
    }
}
