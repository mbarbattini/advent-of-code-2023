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
        if j == N_DIM_2 + 1
            break
        end
        next_char = grid[i, j]
        # clear again
        grid[i, j] = '.'
    end
    return parse(Int, built_num)
end


function solve_part_2()

    total = 0

    for i in 1:N_DIM_1
        for j in 1:N_DIM_2
            current_char = grid[i, j]
            if current_char == '*'
                match_count = 0
                tentative_digit_list = []
                if i > 1 && j > 1
                    upper_left = grid[i-1, j-1]
                end
                if upper_left in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i-1, j-1))
                    # print("$num\n")
                    # total += num
                end
                # because grid gets cleared in read_number, need to get the entry again
                if i > 1
                    top = grid[i-1, j]
                end
                if top in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i-1, j) )
                    # print("$num\n")
                    # total += num
                end 
                if i > 1 && j < N_DIM_2
                    upper_right = grid[i-1, j+1]
                end
                if upper_right in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i-1, j+1))
                    # print("$num\n")
                    # total += num
                end
                if j > 1
                   left = grid[i, j-1]
                end
                if left in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i, j-1) )
                    # print("$num\n")
                    # total += num
                end
                if j < N_DIM_2
                    right = grid[i, j+1]
                end
                if right in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i, j+1))
                    # print("$num\n")
                    # total += num
                end
                if i < N_DIM_1 && j > 1
                    lower_left = grid[i+1, j-1]
                end
                if lower_left in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i+1, j-1))
                    # print("$num\n")
                    # total += num
                end 
                if i < N_DIM_1
                    bottom = grid[i+1, j]
                end
                if bottom in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i+1, j))
                    # print("$num\n")
                    # total += num
                end
                if i < N_DIM_1 && j < N_DIM_2
                    lower_right = grid[i+1, j+1]
                end
                if lower_right in digits
                    match_count += 1
                    append!(tentative_digit_list, read_number(grid, i+1, j+1))
                    # print("$num\n")
                    # total += num
                end 
                # check if only 2 matches
                if match_count == 2
                    total += reduce(*, tentative_digit_list)
                end
            end
        end
    end
    return total
end


solve_part_2()

