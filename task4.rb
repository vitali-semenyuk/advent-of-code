def check_boards(boards)
  boards.each.with_index do |board, index|
    return index if board.any? { _1.all?(&:nil?) }
    return index if board.transpose.any? { _1.all?(&:nil?) }
  end
  nil
end

input = File.read('input4.txt').split("\n\n")

numbers = input.first.split(',')
boards = input.drop(1).map { _1.split("\n").map(&:split) }

def solve(numbers, boards, first = true)
  index = nil
  numbers.each do |number|
    boards.each do |board|
      board.each do |row|
        index = row.index(number)
        row[index] = nil if index
      end
    end

    if first
      index = check_boards(boards)
      return [boards[index], number] if index
    else
      board = nil
      while index = check_boards(boards) do
        board = boards.delete_at(index)
      end
      return [board, number] if boards.empty?
    end
  end
end

board, last_number = solve(numbers, boards)
score = board.flatten.map(&:to_i).sum * last_number.to_i
puts score

board, last_number = solve(numbers, boards, false)
score = board.flatten.map(&:to_i).sum * last_number.to_i
puts score

require 'pry'
binding.pry


