# frozen_string_literal: true

gamma = lines.map(&:chars).transpose.map(&:tally).map { |hash| hash.max_by { _2 }.first }.join.to_i(2)
epsilon = lines.map(&:chars).transpose.map(&:tally).map { |hash| hash.min_by { _2 }.first }.join.to_i(2)

p1 gamma * epsilon

def solve(input, most = true)
  input = input.dup
  size = input.first.size
  (0...size).each do |i|
    bit = input.map { _1[i] }.tally.then do
      next _1.first.first if _1.size == 1

      if _1['1'] > _1['0']
        most ? '1' : '0'
      elsif _1['1'] < _1['0']
        most ? '0' : '1'
      else
        most ? '1' : '0'
      end
    end
    input.filter! { _1[i] == bit }
  end
  input.first.to_i(2)
end

oxygen = solve(lines)
co2 = solve(lines, false)

p2 oxygen * co2

__END__
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
