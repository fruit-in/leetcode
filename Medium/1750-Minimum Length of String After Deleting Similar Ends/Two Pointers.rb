# @param {String} s
# @return {Integer}
def minimum_length(s)
  i = 0
  j = s.size - 1

  while i < j && s[i] == s[j]
    c = s[i]
    i += 1 while i < j && s[i] == c
    return 0 if i == j
    j -= 1 while i < j && s[j] == c
  end

  j - i + 1
end
