# @param {Integer[]} nums
# @return {Integer}
def tuple_same_product(nums)
  counter = {}
  counter.default = 0

  (0...nums.size).each do |i|
    (i + 1...nums.size).each do |j|
      counter[nums[i] * nums[j]] += 1
    end
  end

  counter.values.map { |c| c * (c - 1) * 4 }.sum
end
