impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr3: Vec<i32> = (0..=1000)
            .filter(|n| arr1.contains(n) && !arr2.contains(n))
            .collect();
 
        let mut arr2: Vec<i32> = arr2;
        arr2.append(&mut arr3);
 
        let counter: Vec<usize> = arr2.iter()
            .map(|n| arr1.iter().filter(|&m| m == n).count())
            .collect();
 
        let mut ret: Vec<i32> = Vec::new();
        for i in 0..arr2.len() {
            for j in 0..counter[i] {
                ret.push(arr2[i]);
            }
        }
        ret
    }
}
