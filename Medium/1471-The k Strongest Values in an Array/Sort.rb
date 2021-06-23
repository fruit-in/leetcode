# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer[]}
def get_strongest(arr, k)
  arr.sort!
  m = (arr.size - 1) / 2
  i = 0
  j = arr.size - 1
  ret = []

  while ret.size < k
    if (arr[i] - arr[m]).abs > (arr[j] - arr[m]).abs
      ret.push(arr[i])
      i += 1
    else
      ret.push(arr[j])
      j -= 1
    end
  end

  ret
end
