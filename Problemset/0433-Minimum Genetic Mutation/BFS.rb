# @param {String} start
# @param {String} end
# @param {String[]} bank
# @return {Integer}
def min_mutation(start, end_, bank)
  seen = Set[start]
  genes = Containers::Queue.new([[start, 0]])

  until genes.empty?
    gene, n = genes.pop
    return n if gene == end_

    (0..7).each do |i|
      'ACGT'.each_char do |c|
        new_gene = gene[0...i] + c + gene[i + 1..]
        if bank.member?(new_gene) && !seen.member?(new_gene)
          seen.add(new_gene)
          genes.push([new_gene, n + 1])
        end
      end
    end
  end

  -1
end
