# @param {Integer[]} encoded
# @param {Integer} first
# @return {Integer[]}
def decode(encoded, first)
  ret = [first]

  encoded.each do |n|
    first ^= n
    ret.push(first)
  end

  ret
end
