impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;

        for i in (0..arr.len() - 1).rev() {
            if arr[i] > arr[i + 1] {
                let mut j = i + 1;

                for k in i + 2..arr.len() {
                    if arr[j] < arr[k] && arr[i] > arr[k] {
                        j = k;
                    }
                }

                arr.swap(i, j);

                break;
            }
        }

        arr
    }
}
