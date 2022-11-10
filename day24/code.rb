num = 36_969_794_979_199
i = num.to_s.chars.map(&:to_i)

w = i[0]
x = 1
y = i[0] + 6
z = i[0] + 6

pp(w: w, x: x, y: y, z: z)

w = i[1]
x = 1
y = i[1] + 12
z = (i[0] + 6) * 26 + i[1] + 12

pp(w: w, x: x, y: y, z: z)

w = i[2]
x = 1
y = i[2] + 8
z = ((i[0] + 6) * 26 + i[1] + 12) * 26 + i[2] + 8

pp(w: w, x: x, y: y, z: z)

w = i[3]
if (i[2] - 3) != i[3]
  x = 1
  y = i[3] + 7
  z = ((i[0] + 6) * 26 + i[1] + 12) * 26 + i[3] + 7
else # need this
  x = 0
  y = 0
  z = (i[0] + 6) * 26 + i[1] + 12
end

pp(w: w, x: x, y: y, z: z)

w = i[4]
x = 1
y = i[4] + 7
z = z * 26 + i[4] + 7
# z = ((i[0] + 6) * 26 + i[1] + 12) * 26 + i[4] + 7

pp(w: w, x: x, y: y, z: z)

w = i[5]
x = 1
y = i[5] + 12
z = z * 26 + i[5] + 12
# z = (((i[0] + 6) * 26 + i[1] + 12) * 26 + i[4] + 7) * 26 + i[5] + 12

pp(w: w, x: x, y: y, z: z)

w = i[6]
x = 1
y = i[6] + 2
z = z * 26 + i[6] + 2
# z = ((((i[0] + 6) * 26 + i[1] + 12) * 26 + i[4] + 7) * 26 + i[5] + 12) * 26 + i[6] + 2

pp(w: w, x: x, y: y, z: z)

w = i[7]
if (i[6] - 5) != i[7]
  x = 1
  y = i[7] + 15
  z = z - i[6] + i[7] + 13
else # need this
  x = 0
  y = 0
  z = z / 26
end
# z = (((i[0] + 6) * 26 + i[1] + 12) * 26 + i[4] + 7) * 26 + i[5] + 12

pp(w: w, x: x, y: y, z: z)

w = i[8]
x = 1
y = i[8] + 4
z = z * 26 + i[8] + 4
# z = ((((i[0] + 6) * 26 + i[1] + 12) * 26 + i[4] + 7) * 26 + i[5] + 12) * 26 + i[8] + 4

pp(w: w, x: x, y: y, z: z)

w = i[9]
if (i[8] - 2) != i[9]
  x = 1
  y = i[9] + 5
  z = z - i[8] + i[9] + 1
else # need this
  x = 0
  y = 0
  z = z / 26
end
# z = (((i[0] + 6) * 26 + i[1] + 12) * 26 + i[4] + 7) * 26 + i[5] + 12

pp(w: w, x: x, y: y, z: z)

w = i[10]
if (z % 26 - 10) != i[10] # need this
  x = 1
  y = i[10] + 12
  z = z / 26 * 26 + i[10] + 12
else
  x = 0
  y = 0
  z = z / 26
end
# z = ((i[0] + 6) * 26 + i[1] + 12) * 26 + i[4] + 7

pp(w: w, x: x, y: y, z: z)

w = i[11]
if (z % 26 - 15) != i[11]
  x = 1
  y = i[11] + 11
  z = z / 26 * 26 + i[11] + 11
else # need this
  x = 0
  y = 0
  z = z / 26
end
# z = (i[0] + 6) * 26 + i[1] + 12

pp(w: w, x: x, y: y, z: z)

w = i[12]
if (z % 26 - 9) != i[12]
  x = 1
  y = i[12] + 13
  z = z / 26 * 26 + i[12] + 13
else # need this
  x = 0
  y = 0
  z = z / 26
end
# z = i[0] + 6

pp(w: w, x: x, y: y, z: z)

w = i[13]
if (z % 26) != i[13]
  x = 1
  y = i[13] + 7
  z = z / 26 * 26 + i[13] + 7
else # need this
  x = 0
  y = 0
  z = z / 26
end
# z = 0

pp(w: w, x: x, y: y, z: z)
