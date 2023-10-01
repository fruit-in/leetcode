impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        let money = money - children;

        if money < 0 {
            -1
        } else if money / 7 == children && money % 7 == 0 {
            children
        } else if money / 7 >= children {
            children - 1
        } else if money / 7 == children - 1 && money % 7 == 3 {
            children - 2
        } else {
            money / 7
        }
    }
}
