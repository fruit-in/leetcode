# @param {String[]} names
# @return {String[]}
def get_folder_names(names)
    h = Hash.new(0)

    for i in 0...names.length
        if h[names[i]] > 0
            while h.key?(names[i] + "(%d)" % h[names[i]])
                h[names[i]] += 1
            end
            names[i] += "(%d)" % h[names[i]]
        end
        h[names[i]] += 1
    end

    return names
end
