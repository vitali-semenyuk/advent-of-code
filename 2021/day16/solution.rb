# frozen_string_literal: true

def parse_packet(packet)
  version = packet[..2].to_i(2)
  type_id = packet[3..5].to_i(2)

  parsed = { version: version, type_id: type_id }

  if type_id == 4
    bits = []
    i = 6
    packet[6..].chars.each_slice(5) do |group|
      f = group.first
      payload = group[1..]
      bits << payload

      i += 5
      break if f == '0'
    end

    parsed[:payload] = bits.join.to_i(2)
    parsed[:index] = i
  else
    subpackets = []

    length_type_id = packet[6]
    if length_type_id == '0'
      len = packet[7...(7 + 15)].to_i(2)
      i = 22
      loop do
        subpackets << parse_packet(packet[i..])
        i += subpackets.last[:index]
        break if i - 22 >= len
      end
    else
      num = packet[7...(7 + 11)].to_i(2)
      i = 18
      loop do
        subpackets << parse_packet(packet[i..])
        i += subpackets.last[:index]
        num -= 1
        break if num.zero?
      end
    end
    parsed[:index] = i

    parsed[:subpackets] = subpackets
  end

  parsed
end

def sol(packet)
  res = 0
  res += packet[:version]
  sub = packet[:subpackets] || []
  sub.each { res += sol(_1) }
  res
end

def evalu(packet)
  sub = packet[:subpackets]&.map { evalu(_1) }
  case packet[:type_id]
  when 0
    sub.sum
  when 1
    sub.inject(:*)
  when 2
    sub.min
  when 3
    sub.max
  when 4
    packet[:payload]
  when 5
    sub.then { _1 > _2 ? 1 : 0 }
  when 6
    sub.then { _1 < _2 ? 1 : 0 }
  when 7
    sub.then { _1 == _2 ? 1 : 0 }
  end
end

lines.each do |line|
  puts line
  val = line.strip.chars.map(&:hex).map { _1.to_s(2).rjust(4, '0') }.join
  parsed = parse_packet(val)
  p1 sol(parsed)

  p2 evalu(parsed)

  puts
end

__END__
D2FE28
38006F45291200
EE00D40C823060
8A004A801A8002F478
620080001611562C8802118E34
C0015000016115A2E0802F182340
A0016C880162017C3686B18A3D4780
C200B40A82
04005AC33890
880086C3E88112
CE00C43D881120
D8005AC2A8F0
F600BC2D8F
9C005AC2F8F0
9C0141080250320F1802104A08
