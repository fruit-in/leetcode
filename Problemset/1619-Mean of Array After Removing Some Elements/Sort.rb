# @param {Integer[]} arr
# @return {Float}
def trim_mean(arr)
  length = arr.length

  arr.sort!
  arr = arr[length / 20...-length / 20]

  arr.sum / (length * 0.9)
end
