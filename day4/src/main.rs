use std::fs;

fn main() {
    let mut grid = vec![vec!['.'; 137]; 137];
    let input = fs::read_to_string("input.txt").expect("Could not open file");
    let mut totalSum = 0;

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            //do the things
            println!("char {} at: i: {}, j: {}", char, i, j);
            grid[i][j] = char;
        }
    }

    for (i, line) in grid.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            let mut localSum = 0;

            // we need to handle boundaries
            // if the char is at the edge there will be nothing to check...
            /*
             * x x x
             * x x x
             * x x x
             */
            /*
             * the coords are (i,j)
             * (0,0), (0,1), (0,2)
             * (1,0), (1,1), (1,2)
             * (2,0), (2,1), (2,2)
             *
             *  if i == 0, don't check above
             *  if j == 0, don't check to left
             *  if i == max, don't check below
             *  if j == max, don't check to right
             *
             *  hold on it's more complicated than that
             *  we need to know if we can get the cell above and to the left
             *  above to the right
             *  below to the left
             *  and below to the right
             */
            if *char == '@' {
                if i != 0 {
                    localSum += checkAbove(&grid, i, j);
                }
                if j != 0 {
                    localSum += checkLeft(&grid, i, j);
                }

                if i != 136 {
                    localSum += checkBelow(&grid, i, j);
                }
                if j != 136 {
                    localSum += checkRight(&grid, i, j);
                }

                if i != 0 && j != 0 {
                    localSum += checkTopLeft(&grid, i, j);
                }

                if i != 0 && j != 136 {
                    localSum += checkTopRight(&grid, i, j);
                }

                if i != 136 && j != 0 {
                    localSum += checkBottomLeft(&grid, i, j);
                }

                if i != 136 && j != 136 {
                    localSum += checkBottomRight(&grid, i, j);
                }
                if localSum > 4 {
                    totalSum += 1;
                }
            }
        }
    }
    println!("total sum = {}", totalSum);
}

fn checkAbove(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i - 1][j] == '@' { 1 } else { 0 }
}

fn checkLeft(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i][j - 1] == '@' { 1 } else { 0 }
}

fn checkBelow(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i + 1][j] == '@' { 1 } else { 0 }
}

fn checkRight(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i][j + 1] == '@' { 1 } else { 0 }
}

fn checkTopLeft(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i - 1][j - 1] == '@' { 1 } else { 0 }
}

fn checkTopRight(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i - 1][j + 1] == '@' { 1 } else { 0 }
}

fn checkBottomLeft(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i + 1][j - 1] == '@' { 1 } else { 0 }
}

fn checkBottomRight(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    if grid[i + 1][j + 1] == '@' { 1 } else { 0 }
}
