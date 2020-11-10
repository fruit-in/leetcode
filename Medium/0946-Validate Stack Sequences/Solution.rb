# @param {Integer[]} pushed
# @param {Integer[]} popped
# @return {Boolean}
def validate_stack_sequences(pushed, popped)
    stack = []
    i = 0

    for val in pushed
        stack.push(val)
        while i < popped.length and stack.last == popped[i]
            stack.pop
            i += 1
        end
    end

    return stack.empty?
end
