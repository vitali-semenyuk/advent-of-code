# frozen_string_literal: true

brackets = {
  '(' => ')',
  '[' => ']',
  '{' => '}',
  '<' => '>'
}.invert.freeze

points = {
  ')' => 3,
  ']' => 57,
  '}' => 1197,
  '>' => 25_137
}

invalid = []

incomplete = lines.map do |line|
  stack = []
  inv = false
  line.chars.each do |char|
    if brackets.values.include?(char)
      stack.push(char)
    elsif stack.last == brackets[char]
      stack.pop
    else
      invalid.push(char)
      inv = true
      break
    end
  end

  if inv
    nil
  else
    stack.map { brackets.invert.fetch(_1) }.reverse
  end
end.compact

p1 invalid.reduce(0) { |acc, char| acc + points[char] }

points = incomplete.map do |line|
  score = 0
  line.each do |char|
    score *= 5
    score += brackets.keys.index(char) + 1
  end
  score
end

p2 points.sort[points.size / 2]

__END__
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
