# frozen_string_literal: true

Node = Struct.new(:left, :right, :value, :level, :parent, keyword_init: true)

def parse_num(str)
  stack = []
  level = 0

  str.chars.each do |char|
    case char
    when '['
      stack << Node.new(level: level, parent: stack.last)
      level += 1
    when ']'
      if stack.size > 1
        val = stack.pop
        if stack.last.left
          stack.last.right = val
        else
          stack.last.left = val
        end
      end
      level -= 1
    when ','
    when '0'..'9'
      n = Node.new(value: char.to_i, level: level, parent: stack.last)
      if stack.last.left
        stack.last.right = n
      else
        stack.last.left = n
      end
    else
      raise 'Error'
    end
  end

  stack.first
end

def reduce(num)
  reduced = false

  loop do
    pair = find_explode(num)
    break unless pair

    explode(pair)

    reduced = true
  end

  pair = find_split(num)
  if pair
    split(pair)
    reduced = true
  end

  reduced ? reduce(num) : num
end

def find_explode(num)
  res = nil

  return num if num.level >= 4

  res = find_explode(num.left) if num.left.value.nil?
  res ||= find_explode(num.right) if num.right.value.nil?

  res
end

def succ(n, s)
  return min_value(n.right) if n.right && n.right.__id__ != s.__id__

  p = n.parent
  loop do
    break unless p
    break if n != p.right

    n = p
    p = p.parent
  end

  min_value(p&.right)
end

def min_value(node)
  current = node

  loop do
    break unless current
    break unless current.left

    current = current.left
  end

  current
end

def prev(n, s)
  return max_value(n.left) if n.left && n.left.__id__ != s.__id__

  p = n.parent
  loop do
    break unless p
    break if n != p.left

    n = p
    p = p.parent
  end

  max_value(p&.left)
end

def max_value(node)
  current = node

  loop do
    break unless current
    break unless current.right

    current = current.right
  end

  current
end

def explode(pair)
  l = pair.left
  r = pair.right
  ll = prev(pair.parent, pair)
  ll.value += l.value if ll

  rr = succ(pair.parent, pair)
  rr.value += r.value if rr

  pair.left = nil
  pair.right = nil
  pair.value = 0
end

def find_split(num)
  res = nil

  return num if num.value && num.value > 9

  res = find_split(num.left) if num.left
  res ||= find_split(num.right) if num.right

  res
end

def split(pair)
  pair.left = Node.new(parent: pair, level: pair.level + 1, value: (pair.value / 2.0).floor)
  pair.right = Node.new(parent: pair, level: pair.level + 1, value: (pair.value / 2.0).ceil)
  pair.value = nil
end

def print_num(num)
  return num.value.to_s if num.value

  str = '['
  str += print_num(num.left)
  str += ','
  str += print_num(num.right)
  "#{str}]"
end

def update_levels(num)
  parent_level = num.parent&.level || -1
  num.level = parent_level + 1

  update_levels(num.left) if num.left
  update_levels(num.right) if num.right
end

def sum(a, b)
  parent = Node.new(left: a, right: b)
  a.parent = parent
  b.parent = parent
  update_levels(parent)
  reduce(parent)
  parent
end

def magnitude(num)
  return num.value if num.value

  magnitude(num.left) * 3 + magnitude(num.right) * 2
end

res = lines.map(&method(:parse_num)).reduce do |acc, num|
  sum(acc, num)
end

p1 magnitude(res)

magnitudes = lines.permutation(2).map do
  magnitude(sum(parse_num(_1), parse_num(_2)))
end

p2 magnitudes.max

__END__
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
