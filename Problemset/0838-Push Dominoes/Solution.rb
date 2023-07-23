# @param {String} dominoes
# @return {String}
def push_dominoes(dominoes)
  dominoes = ['L'] + dominoes.chars + ['R']
  i = 0

  (1...dominoes.length).each do |j|
    next if dominoes[j] == '.'

    if dominoes[i] == dominoes[j]
      (i...j).each do |k|
        dominoes[k] = dominoes[i]
      end
    elsif dominoes[i] == 'R' && dominoes[j] == 'L'
      (1..(j - i - 1) / 2).each do |k|
        dominoes[i + k] = 'R'
        dominoes[j - k] = 'L'
      end
    end

    i = j
  end

  dominoes[1...-1].join
end
