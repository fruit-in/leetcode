# @param {String[]} paths
# @return {String[][]}
def find_duplicate(paths)
    h = Hash.new {|hash, key| hash[key] = []}

    for path in paths
        path = path.split(' ')

        for file in path[1..]
            i = file.index('(')

            file_path = path[0] + '/' + file[0...i]
            content = file[(i + 1)...-1]

            h[content].push(file_path)
        end
    end

    return h.values.select {|elem| elem.length > 1}
end
