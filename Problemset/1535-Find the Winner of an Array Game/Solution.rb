# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def get_winner(arr, k)
  winner = arr[0]
  wins = 0

  arr[1..].each do |x|
    if x > winner
      winner = x
      wins = 0
    end
    wins += 1
    break if wins == k
  end

  winner
end
