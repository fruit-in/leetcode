# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def num_subarray_product_less_than_k(nums, k)
  return 0 if k < 2

  product = 1
  i = 0
  ret = 0

  (0...nums.size).each do |j|
    product *= nums[j]
    while product >= k
      product /= nums[i]
      i += 1
    end
    ret += j - i + 1
  end

  ret
end
