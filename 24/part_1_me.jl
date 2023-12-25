using Combinatorics

filename = "/Users/matthewbarbattini/Desktop/AdventOfCode2023/24/input.txt"

MAX_POS = 400000000000000
MIN_POS = 200000000000000


function parsedata(line)
    p, v = split(line, " @ ")
    px, py, pz = split(p, ", ") .|> val -> parse(Int, val)
    vx, vy, vz = split(v, ", ") .|> val -> parse(Int, val)
    return ((px, py, pz), (vx, vy, vz))
end

stones = readlines(filename) .|> parsedata


total = 0

combos = combinations(stones, 2) .|> collect
for pair in combos
    stone1 = pair[1]
    stone2 = pair[2]
    pos1 = stone1[1]
    vel1 = stone1[2]
    pos2 = stone2[1]
    vel2 = stone2[2]


    m1 = vel1[2] / vel1[1]
    m2 = vel2[2] / vel2[1]

    b1 = pos1[2] - m1 * pos1[1]
    b2 = pos2[2] - m2 * pos2[1]

    x_intersect = (b1 - b2) / (m2 - m1)
    # use the first line to extrapolate to intersect
    y_intersect = m1 * x_intersect + b1

    t_intersect_1 = (x_intersect - pos1[1]) / vel1[1]
    t_intersect_2 = (x_intersect - pos2[1]) / vel2[1]

    if MIN_POS < x_intersect && x_intersect < MAX_POS && MIN_POS < y_intersect && y_intersect < MAX_POS && t_intersect_1 > 0 && t_intersect_2 > 0
        global total += 1
    end




end
total