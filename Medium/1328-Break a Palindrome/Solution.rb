# @param {String} palindrome
# @return {String}
def break_palindrome(palindrome)
  return '' if palindrome.size == 1

  (0...palindrome.size / 2).each do |i|
    if palindrome[i] != 'a'
      palindrome[i] = 'a'
      return palindrome
    end
  end

  palindrome[-1] = 'b'

  palindrome
end
