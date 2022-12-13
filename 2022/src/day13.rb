require 'json'

data = <<-STR
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
STR
data = File.read('./tasks/day13.txt')

def ordered(left, right)
  if left.is_a?(Numeric) && right.is_a?(Numeric)
    return -1 if left < right
    return 1 if left > right
    0
  elsif left.is_a?(Array) && right.is_a?(Array)
    done = left.zip(right).filter { |l, r| r }.map { |l, r| ordered(l, r) }.find { _1 != 0 }
    return done if done
    return -1 if left.size < right.size
    return 1 if left.size > right.size

    0
  else
    left.is_a?(Numeric) ? ordered([left], right) : ordered(left, [right])
  end
end

pairs = data.split("\n\n").map { _1.lines.map(&JSON.method(:parse)) }
results = pairs.map { |f, s| ordered(f, s) }
pp results.map.with_index { |r, i| [r, i + 1] }.filter { |r, _| r == -1 }.sum { |r, i| i }

pairs.push([ [[2]], [[6]] ])
sorted = pairs.flatten(1).sort { |f, s| ordered(f, s) }

pp (sorted.index([[2]]) + 1) * (sorted.index([[6]]) + 1)
