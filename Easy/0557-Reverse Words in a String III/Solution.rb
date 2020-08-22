# @param {String} s
# @return {String}
def reverse_words(s)
    return s.split.map {|word| word.reverse}.join(' ')
end
