# @param {String} sequence
# @param {String} word
# @return {Integer}
def max_repeating(sequence, word)
  i = 0
  k = 0

  while i + (k + 1) * word.length <= sequence.length
    if sequence[i...i + (k + 1) * word.length] == word * (k + 1)
      k += 1
    else
      i += 1
    end
  end

  k
end
