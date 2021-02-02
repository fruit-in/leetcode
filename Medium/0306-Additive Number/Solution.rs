impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        for i in 1..(num.len() + 1) / 2 {
            if num.get(..1).unwrap() == "0" && i > 1 {
                break;
            }

            for j in 1..=((num.len() - i) / 2).min(num.len() - 2 * i) {
                if num.get(i..i + 1).unwrap() == "0" && j > 1 {
                    break;
                }

                let mut k = i + j;
                let mut x = num.get(..i).unwrap().parse::<u64>().unwrap();
                let mut y = num.get(i..k).unwrap().parse::<u64>().unwrap();
                let mut l = k + (x + y).to_string().len();
                let mut z;

                while l <= num.len() {
                    z = num.get(k..l).unwrap().parse::<u64>().unwrap();

                    if x + y != z {
                        break;
                    }

                    x = y;
                    y = z;
                    k = l;
                    l = k + (x + y).to_string().len();
                }

                if k == num.len() {
                    return true;
                }
            }
        }

        false
    }
}
