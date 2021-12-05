# frozen_string_literal: true

digits = lines.map do |line|
  line.split('|').map(&:split)
end

diff = ->(a, b) { a.chars - b.chars }

p1 digits.map(&:last).flatten.count { [2, 4, 3, 7].include? _1.size }

hash = {
  0 => 'abcefg',
  1 => 'cf',
  2 => 'acdeg',
  3 => 'acdfg',
  4 => 'bcdf',
  5 => 'abdfg',
  6 => 'abdefg',
  7 => 'acf',
  8 => 'abcdefg',
  9 => 'abcdfg'
}.freeze
res = input.lines.map do |line|
  learn, target = line.split('|').map(&:split)

  mapping = learn.map do |num|
    [num, hash.values.index { _1.size == num.size }]
  end.select { [1, 4, 7, 8].include? _2 }.to_h.invert

  h = {}
  h [diff.call(hash[7], hash[1]).first] = diff.call(mapping[7], mapping[1]).first

  mapping[6] = learn.select { _1.size == hash[0].size && diff.call(mapping[1], _1).any? }.first
  h [diff.call(hash[8], hash[6]).first] = diff.call(mapping[1], mapping[6]).first
  h[diff.call(hash[1], diff.call(hash[1], hash[6]).first).first] =
    diff.call(mapping[1], diff.call(mapping[1], mapping[6]).first).first

  mapping[5] = learn.select { _1.size == hash[5].size && !_1.include?(h['c']) }.first

  mapping[2] = learn.select { _1.size == hash[2].size && !_1.include?(h['f']) }.first

  h [diff.call(hash[6], hash[5]).first] = diff.call(mapping[6], mapping[5]).first

  mapping[9] = learn.select { _1.size == hash[9].size && !_1.include?(h['e']) }.first

  mapping[0] = learn.select { _1.size == hash[0].size && !mapping.values.include?(_1) }.first
  mapping[3] = learn.select { _1.size == hash[3].size && !mapping.values.include?(_1) }.first
  mapping.transform_values! { _1.chars.sort.join }

  target.map { mapping.invert[_1.chars.sort.join] }.join.to_i
end

p2 res.sum

__END__
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
