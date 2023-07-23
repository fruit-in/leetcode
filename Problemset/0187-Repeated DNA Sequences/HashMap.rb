# @param {String} s
# @return {String[]}
def find_repeated_dna_sequences(s)
  counter = {}
  counter.default = 0

  (0..s.length - 10).each do |i|
    counter[s[i...i + 10]] += 1
  end

  counter.filter { |_, c| c > 1 }.keys
end
