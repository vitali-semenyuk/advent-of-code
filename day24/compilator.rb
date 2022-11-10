source = File.read('day24/input.txt').lines.map(&:strip)

translated = source.map do |line|
  command, *operands = line.split
  a, b = operands

  case command
  when 'inp'
    "\n#{a} = input"
  when 'add'
    "#{a} = #{a} + #{b}"
  when 'mul'
    "#{a} = #{a} * #{b}"
  when 'div'
    "#{a} = #{a} / #{b}"
  when 'mod'
    "#{a} = #{a} % #{b}"
  when 'eql'
    "#{a} = #{a} == #{b}"
  else
    'undef'
  end
end

File.write 'day24/code.txt', translated.join("\n")
