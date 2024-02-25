impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let ones = arr.iter().filter(|&&x| x == 1).count() as i32;

        if ones % 3 != 0 {
            return vec![-1, -1];
        } else if ones == 0 {
            return vec![0, 2];
        }

        let mut i = 0;
        let mut j = 0;
        let mut k = arr.len() - 1;
        let mut count = 0;

        while arr[k] == 0 {
            k -= 1;
        }

        while count < ones / 3 {
            count += arr[i];
            i += 1;
        }
        for _ in k..arr.len() - 1 {
            if arr[i] == 1 {
                return vec![-1, -1];
            }

            i += 1;
        }
        i -= 1;

        j = i + 1;
        count = 0;
        while count < ones / 3 {
            count += arr[j];
            j += 1;
        }
        for _ in k..arr.len() - 1 {
            if arr[j] == 1 {
                return vec![-1, -1];
            }

            j += 1;
        }

        for k in 0..(i + 1).min(arr.len() - j).min(j - i - 1) {
            let (a, b, c) = (arr[i - k], arr[j - 1 - k], arr[arr.len() - 1 - k]);
            if a != b || b != c || a != c {
                return vec![-1, -1];
            }
        }

        vec![i as i32, j as i32]
    }
}
