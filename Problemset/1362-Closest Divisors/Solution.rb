# @param {Integer} num
# @return {Integer[]}
def closest_divisors(num)
    ret = [1, num + 1]

    for n in (num + 1)..(num + 2)
        for i in (Integer.sqrt(n)...1).step(-1)
            if n % i == 0 and n / i - i < ret[1] - ret[0]
                ret = [i, n / i]
                break
            end
        end
    end

    return ret
end
