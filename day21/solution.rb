# frozen_string_literal: true

first, second = lines
first = first.split(':').last.to_i - 1
second = second.split(':').last.to_i - 1

rolls = 0
first_score = 0
second_score = 0
dice = (1..100).cycle

loop do
  dice_throws = 3.times.map { dice.next }
  points = dice_throws.sum
  first += points
  first %= 10
  first_score += first + 1

  puts "Player 1 rolls #{dice_throws.join('+')} and moves to space #{first + 1} for a total score of #{first_score}"

  rolls += 3
  break if first_score >= 1000

  dice_throws = 3.times.map { dice.next }
  points = dice_throws.sum
  second += points
  second %= 10
  second_score += second + 1

  rolls += 3
  puts "Player 2 rolls #{dice_throws.join('+')} and moves to space #{second + 1} for a total score of #{second_score}"

  break if second_score >= 1000
end

p1 [first_score, second_score].min * rolls

res = 0
p2 res

__END__
Player 1 starting position: 4
Player 2 starting position: 8
