"""
Cases not dealt with:
2. Number is at the edge of the input. Going to index outside range when building
      .......522
      ......*...
      OR
      352.......
      ...*......
3. Number is close to more than one symbol
      ...*......
      3524......
      *.........
4. . added to number when it shouldn't be
"""


lines = readlines("/Users/matthewbarbattini/Desktop/AdventOfCode2023/day_3/src/input.txt")


N_DIM_1 = length(lines[1])
N_DIM_2 = length(lines)

grid = fill(' ', (N_DIM_1, N_DIM_2))
for i in 1:N_DIM_1
    for j in 1:N_DIM_2
        current_char = Char(lines[i][j])
        grid[i, j] = lines[i][j]
    end
end

grid

symbols = [
        '@',
        '#',
        '$',
        '%',
        '&',
        '*',
        '+',
        '=',
        '/',
        '-'
]    

digits = [
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9'
]

function read_number(grid, i, j)

    # go to the left until there are no more digits
    j -= 1
    next_char = grid[i, j]
    while next_char in digits
        if !(next_char in digits)
            break
        end
        j -= 1
        # check if this number goes all the way to the left column
        if j == 0
            break
        end
        next_char = grid[i, j]
    end
    # j is at the first index, build the number to the right until no more digits
    built_num = ""
    j += 1
    next_char = grid[i, j]
    # clear the number at the beginning
    grid[i, j] = '.'
    while next_char in digits
        built_num = built_num * string(next_char)
        j += 1
        # check if number goes all the way to the right column
        if j == N_DIM_2
            break
        end
        next_char = grid[i, j]
        # clear again
        grid[i, j] = '.'
    end
    return parse(Int, built_num)
end





function solve_part_1()

    total = 0

    for i in 1:N_DIM_1
        for j in 1:N_DIM_2
            current_char = grid[i, j]
            # println(current_char)
            if current_char in symbols
                # println(grid[i-1,j-2:j+2])
                # println(grid[i+1,j-2:j+2])
                # println(grid[i,j-2:j+2])
                # println("")
                if i > 1 && j > 1
                    upper_left = grid[i-1, j-1]
                end
                if upper_left in digits
                    num = read_number(grid, i-1, j-1)
                    print("$num\n")
                    total += num
                end
                # because grid gets cleared in read_number, need to get the entry again
                if i > 1
                    top = grid[i-1, j]
                end
                if top in digits
                    num = read_number(grid, i-1, j)
                    print("$num\n")
                    total += num
                end 
                if i > 1 && j < N_DIM_2
                    upper_right = grid[i-1, j+1]
                end
                if upper_right in digits
                    num = read_number(grid, i-1, j+1)
                    print("$num\n")
                    total += num
                end
                if j > 1
                   left = grid[i, j-1]
                end
                if left in digits
                    num = read_number(grid, i, j-1)
                    print("$num\n")
                    total += num
                end
                if j < N_DIM_2
                    right = grid[i, j+1]
                end
                if right in digits
                    num = read_number(grid, i, j+1)
                    print("$num\n")
                    total += num
                end
                if i < N_DIM_1 && j > 1
                    lower_left = grid[i+1, j-1]
                end
                if lower_left in digits
                    num = read_number(grid, i+1, j-1)
                    print("$num\n")
                    total += num
                end 
                if i < N_DIM_1
                    bottom = grid[i+1, j]
                end
                if bottom in digits
                    num = read_number(grid, i+1, j)
                    print("$num\n")
                    total += num
                end
                if i < N_DIM_1 && j < N_DIM_2
                    lower_right = grid[i+1, j+1]
                end
                if lower_right in digits
                    num = read_number(grid, i+1, j+1)
                    print("$num\n")
                    total += num
                end 

            end
        end
        # print("\n")
    end
    return total
end
# println(grid)
solve_part_1()