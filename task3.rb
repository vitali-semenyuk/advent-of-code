input = File.read('input3.txt').lines.map(&:strip)

gamma = input.map { _1.split('') }.transpose.map(&:tally).map { |hash| hash.sort_by { _2 }.last.first  }.join.to_i(2)
epsilon = input.map { _1.split('') }.transpose.map(&:tally).map { |hash| hash.sort_by { _2 }.first.first  }.join.to_i(2)


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

oxygen = solve(input)
co2 = solve(input, false)

require 'pry'
binding.pry


