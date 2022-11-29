# frozen_string_literal: true

commands = lines

x = 0
y = 0

commands.each do |c|
  action, value = c.split
  case action
  when 'forward'
    x += value.to_i
  when 'up'
    y -= value.to_i
  when 'down'
    y += value.to_i
  else
    raise 'Error'
  end
end

p1 x * y

x = 0
y = 0
aim = 0

commands.each do |c|
  action, value = c.split
  case action
  when 'forward'
    x += value.to_i
    y += aim * value.to_i
  when 'up'
    aim -= value.to_i
  when 'down'
    aim += value.to_i
  else
    raise 'Error'
  end
end

p2 x * y

__END__
forward 5
down 5
forward 8
up 3
down 8
forward 2
