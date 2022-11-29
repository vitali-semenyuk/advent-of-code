# frozen_string_literal: true

class ALU
  attr_reader :registers

  def initialize(program)
    @program = program
    @registers = Hash.new(0)
  end

  def calculate(input)
    @program.lines.each do |instruction|
      command, *operands = instruction.split

      case command
      when 'inp'
        pp registers
        @registers[operands.first] = input.shift
      when 'add'
        @registers[operands.first] += val(operands.last)
      when 'mul'
        @registers[operands.first] *= val(operands.last)
      when 'div'
        @registers[operands.first] /= val(operands.last)
      when 'mod'
        @registers[operands.first] %= val(operands.last)
      when 'eql'
        @registers[operands.first] = @registers[operands.first] == val(operands.last) ? 1 : 0
      else
        raise 'Error'
      end
    end
  end

  private

  def val(literal)
    Integer(literal)
  rescue ArgumentError
    @registers[literal]
  end
end

# i[3] = i[2] - 3
# i[7] = i[6] - 5
# i[9] = i[8] - 2
# i[10] = i[5] + 2
# i[11] = i[4] - 8
# i[12] = i[1] + 3
# i[13] = i[0] + 6

def check(num)
  inputs = num.to_s.chars.map(&:to_i)

  alu = ALU.new(input)
  alu.calculate(inputs)

  alu.registers
end

num_max = 36_969_794_979_199
num_min = 11_419_161_313_147

pp check(num_min)
pp check(num_max)

p1 num_max

p2 num_min

__END__
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
