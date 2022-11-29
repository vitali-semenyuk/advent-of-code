# frozen_string_literal: true

template, insertions = input.split("\n\n")
insertions = insertions.lines.map { _1.strip.split(' -> ') }.to_h

def count(pairs)
  chars = Hash.new(0)

  pairs.each do |pair, count|
    chars[pair[0]] += count
    chars[pair[1]] += count
  end

  chars.transform_values { _1 / 2 }
end

def solve(template, insertions, n)
  pairs = template.chars.each_cons(2).map(&:join).tally

  n.times do
    pairs.dup.each do |pair, count|
      new = insertions[pair]
      pairs[pair[0] + new] ||= 0
      pairs[new + pair[1]] ||= 0
      pairs[pair[0] + new] += count
      pairs[new + pair[1]] += count
      pairs[pair] -= count
    end

    pairs.select! { _2.positive? }
  end

  count(pairs)
end

first = solve(template, insertions, 10)
first[template[0]] += 1
first[template[-1]] += 1
sol = first.sort_by { _2 }

p1 sol.last.last - sol.first.last

second = solve(template, insertions, 40)
second[template[0]] += 1
second[template[-1]] += 1
sol = second.sort_by { _2 }

p2 sol.last.last - sol.first.last

__END__
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
