/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7
 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut a = rand7();
        let mut b = rand7() * 7;

        while a + b > 47 {
            a = rand7();
            b = rand7() * 7;
        }

        (a + b + 2) % 10 + 1
    }
}
