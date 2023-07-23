# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def find_least_num_of_unique_ints(arr, k)
  counter = {}
  counter.default = 0
  arr.each { |x| counter[x] += 1 }
  counter = counter.values.sort

  (0..counter.size).each do |i|
    if k == 0
      return counter.size - i
    elsif k < 0
      return counter.size - i + 1
    end

    k -= counter[i]
  end
end
