# frozen_string_literal: true

class ALU
  attr_reader :registers

  def initialize(program)
    @program = program
    @registers = Hash.new(0)
  end

  def calculate(input)
    @program.lines.each do |instruction|
      puts instruction

      command, *operands = instruction.split

      case command
      when 'inp'
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

      pp @registers
    end
  end

  private

  def val(literal)
    Integer(literal)
  rescue ArgumentError
    @registers[literal]
  end
end

alu = ALU.new(input)
alu.calculate((1..9).cycle.first(14))

res = 0

p1 res

p2 res

__END__
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
