# frozen_string_literal: true

first, second = lines
first = first.split(':').last.to_i - 1
second = second.split(':').last.to_i - 1

def part_1(first, second)
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

    rolls += 3
    break if first_score >= 1000

    dice_throws = 3.times.map { dice.next }
    points = dice_throws.sum
    second += points
    second %= 10
    second_score += second + 1

    rolls += 3
    break if second_score >= 1000
  end

  [first_score, second_score].min * rolls
end

p1 part_1(first, second)

first_wins = 0
second_wins = 0

dices = [1, 2, 3].product([1, 2, 3]).product([1, 2, 3]).map(&:flatten).map(&:sum)
dices_var = dices.tally

universes = {
  [first, 0, second, 0] => 1
}
threshold = 21

100.times do
  new_universes = Hash.new(0)
  universes.each do |(pos_f, sco_f, pos_s, sco_s), count|

    dices_var.each do |dice, c|
      new_position = pos_f + dice
      new_position %= 10
      new_score = sco_f + new_position + 1

      new_universes[[new_position, new_score, pos_s, sco_s]] += count * c
    end
  end

  finished, universes = new_universes.entries.partition { |(_, s, _, _), _| s >= threshold }.map(&:to_h)
  first_wins += finished.values.sum

  new_universes = Hash.new(0)
  universes.each do |(pos_f, sco_f, pos_s, sco_s), count|

    dices_var.each do |dice, c|
      new_position = pos_s + dice
      new_position %= 10
      new_score = sco_s + new_position + 1

      new_universes[[pos_f, sco_f, new_position, new_score]] += count * c
    end
  end

  finished, universes = new_universes.entries.partition { |(_, _, _, s), _| s >= threshold }.map(&:to_h)
  second_wins += finished.values.sum
end

p2 first_wins

__END__
Player 1 starting position: 4
Player 2 starting position: 8
