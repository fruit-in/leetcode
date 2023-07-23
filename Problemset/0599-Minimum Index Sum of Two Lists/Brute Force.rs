impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut min_sum = std::usize::MAX;
        let mut ret = Vec::new();

        for i in 0..list1.len() {
            for j in 0..list2.len() {
                if list1[i] == list2[j] {
                    if i + j == min_sum {
                        ret.push(list2[j].to_string());
                    } else if i + j < min_sum {
                        ret.clear();
                        ret.push(list2[j].to_string());
                        min_sum = i + j;
                    }
                    break;
                }
            }
        }

        ret
    }
}
