# @param {Integer} k
# @return {Integer}
def find_min_fibonacci_numbers(k)
  nums = [1, 1]
  ret = 0

  nums.push(nums[-2] + nums[-1]) while nums[-1] < k

  while k > 0
    nums.pop while nums[-1] > k
    k -= nums[-1]
    ret += 1
  end

  ret
end
