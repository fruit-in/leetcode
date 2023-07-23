impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        moves.matches('R').count() == moves.matches('L').count() &&
        moves.matches('U').count() == moves.matches('D').count()
    }
}
