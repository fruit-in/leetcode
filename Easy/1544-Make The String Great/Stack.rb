# @param {String} s
# @return {String}
def make_good(s)
    stack = []

    for ch in s.chars
        if not stack.empty? and (stack[-1].ord - ch.ord).abs == 32
            stack.pop
        else
            stack.push(ch)
        end
    end

    return stack.join
end
