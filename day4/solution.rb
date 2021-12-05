# frozen_string_literal: true

def check_boards(boards)
  boards.each.with_index do |board, index|
    return index if board.any? { _1.all?(&:nil?) }
    return index if board.transpose.any? { _1.all?(&:nil?) }
  end
  nil
end

inp = input.split("\n\n")

numbers = inp.first.split(',')
boards = inp.drop(1).map { _1.split("\n").map(&:split) }

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
      while index = check_boards(boards)
        board = boards.delete_at(index)
      end
      return [board, number] if boards.empty?
    end
  end
end

board, last_number = solve(numbers, boards)
score = board.flatten.map(&:to_i).sum * last_number.to_i
p1 score

board, last_number = solve(numbers, boards, false)
score = board.flatten.map(&:to_i).sum * last_number.to_i
p2 score

__END__
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
