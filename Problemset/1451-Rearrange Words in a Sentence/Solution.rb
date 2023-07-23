# @param {String} text
# @return {String}
def arrange_words(text)
    words = text.downcase.split(' ')
    words.sort_by! {|word| word.length}
    words[0].capitalize!

    return words.join(' ')
end
