impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut flag = false;
        let mut ret = Vec::new();

        for sum in 0..(list1.len() + list2.len() - 1) {
            for i in (sum + 1).saturating_sub(list2.len())..(sum + 1).min(list1.len()) {
                if list1[i] == list2[sum - i] {
                    flag = true;
                    ret.push(list1[i].to_string());
                }
            }

            if flag {
                break;
            }
        }

        ret
    }
}
