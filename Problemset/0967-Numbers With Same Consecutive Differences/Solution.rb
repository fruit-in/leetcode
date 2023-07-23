# @param {Integer} n
# @param {Integer} k
# @return {Integer[]}
def nums_same_consec_diff(n, k)
  nums = (1..9).to_a

  (2..n).each do |_|
    nums_ = []

    nums.each do |x|
      y = x % 10
      nums_.push(x * 10 + y + k) if y + k < 10
      nums_.push(x * 10 + y - k) if y - k >= 0 && k != 0
    end

    nums = nums_
  end

  nums
end
