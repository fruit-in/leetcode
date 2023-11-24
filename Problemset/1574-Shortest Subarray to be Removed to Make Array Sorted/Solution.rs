impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        let mut i = arr.len();
        let mut ret = arr.len() - 1;

        arr.insert(0, 0);

        while i > 0 && arr[i - 1] <= arr[i] {
            i -= 1;
        }

        if i == 0 {
            return 0;
        }

        for j in 0..arr.len() {
            if j > 0 && arr[j - 1] > arr[j] {
                break;
            }

            while i < arr.len() && arr[i] < arr[j] {
                i += 1;
            }

            ret = ret.min(i - j - 1);
        }

        ret as i32
    }
}
