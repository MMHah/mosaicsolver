#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub shaded: Option<bool>,
    pub clue: Option<i32>,
}

pub fn is_on(c: Cell) -> i32 {
    if c.shaded == Some(true) {
        return 1;
    } else {
        return 0;
    };
}

pub fn is_off(c: Cell) -> i32 {
    if c.shaded == Some(false) {
        return 1;
    } else {
        return 0;
    };
}

pub fn is_empty(c: Cell) -> i32 {
    if c.shaded == None {
        return 1;
    } else {
        return 0;
    };
}

pub fn count_shaded_neighbours(grid: &Vec<Cell>, size: usize, curr: usize) -> i32 {
    let mut result = 0;

    // Bigger than size - 1 => not top row, check row above
    if curr > size - 1 {
        // Not left edge, check col to left
        if curr % size != 0 {
            result += is_on(grid[curr - size - 1]);
        }
        // Not right edge, check col to right
        if curr % size != size - 1 {
            result += is_on(grid[curr - size + 1]);
        }
        // All columns check own column
        result += is_on(grid[curr - size]);
    }

    // Smaller than size * (size - 1) => not bottom row, check row below
    if curr < size * (size - 1) {
        if curr % size != 0 {
            result += is_on(grid[curr + size - 1]);
        }
        if curr % size != size - 1 {
            result += is_on(grid[curr + size + 1]);
        }
        result += is_on(grid[curr + size]);
    }

    // All rows check own row
    if curr % size != 0 {
        result += is_on(grid[curr - 1]);
    }
    if curr % size != size - 1 {
        result += is_on(grid[curr + 1]);
    }
    result += is_on(grid[curr]);

    return result;
}

pub fn count_unshaded_neighbours(grid: &Vec<Cell>, size: usize, curr: usize) -> i32 {
    let mut result = 0;
    if curr > size - 1 {
        if curr % size != 0 {
            result += is_off(grid[curr - size - 1]);
        }
        if curr % size != size - 1 {
            result += is_off(grid[curr - size + 1]);
        }
        result += is_off(grid[curr - size]);
    }

    if curr < size * (size - 1) {
        if curr % size != 0 {
            result += is_off(grid[curr + size - 1]);
        }
        if curr % size != size - 1 {
            result += is_off(grid[curr + size + 1]);
        }
        result += is_off(grid[curr + size]);
    }

    if curr % size != 0 {
        result += is_off(grid[curr - 1]);
    }
    if curr % size != size - 1 {
        result += is_off(grid[curr + 1]);
    }
    result += is_off(grid[curr]);

    return result;
}

pub fn count_empty_neighbours(grid: &Vec<Cell>, size: usize, curr: usize) -> i32 {
    let mut result = 0;

    // Top row
    if curr > size - 1 {
        // Left edge
        if curr % size != 0 {
            result += is_empty(grid[curr - size - 1]);
        }
        // Right edge
        if curr % size != size - 1 {
            result += is_empty(grid[curr - size + 1]);
        }
        // Middle
        result += is_empty(grid[curr - size]);
    }

    // Bottom row
    if curr < size * (size - 1) {
        if curr % size != 0 {
            result += is_empty(grid[curr + size - 1]);
        }
        if curr % size != size - 1 {
            result += is_empty(grid[curr + size + 1]);
        }
        result += is_empty(grid[curr + size]);
    }

    // Middle rows
    if curr % size != 0 {
        result += is_empty(grid[curr - 1]);
    }
    if curr % size != size - 1 {
        result += is_empty(grid[curr + 1]);
    }
    result += is_empty(grid[curr]);

    return result;
}

pub fn turn_on_empty_neighbours(grid: &mut Vec<Cell>, size: usize, curr: usize) {
    if curr > size - 1 {
        if curr % size != 0 && grid[curr - size - 1].shaded.is_none() {
            grid[curr - size - 1].shaded = Some(true);
        }
        if curr % size != size - 1 && grid[curr - size + 1].shaded.is_none() {
            grid[curr - size + 1].shaded = Some(true);
        }
        if grid[curr - size].shaded.is_none() {
            grid[curr - size].shaded = Some(true);
        }
    }

    if curr < size * (size - 1) {
        if curr % size != 0 && grid[curr + size - 1].shaded.is_none() {
            grid[curr + size - 1].shaded = Some(true);
        }
        if curr % size != size - 1 && grid[curr + size + 1].shaded.is_none() {
            grid[curr + size + 1].shaded = Some(true);
        }
        if grid[curr + size].shaded.is_none() {
            grid[curr + size].shaded = Some(true);
        }
    }

    if curr % size != 0 && grid[curr - 1].shaded.is_none() {
        grid[curr - 1].shaded = Some(true);
    }
    if curr % size != size - 1 && grid[curr + 1].shaded.is_none() {
        grid[curr + 1].shaded = Some(true);
    }
    if grid[curr].shaded.is_none() {
        grid[curr].shaded = Some(true);
    }
}

pub fn turn_off_empty_neighbours(grid: &mut Vec<Cell>, size: usize, curr: usize) {
    if curr > size - 1 {
        if curr % size != 0 && grid[curr - size - 1].shaded.is_none() {
            grid[curr - size - 1].shaded = Some(false);
        }
        if curr % size != size - 1 && grid[curr - size + 1].shaded.is_none() {
            grid[curr - size + 1].shaded = Some(false);
        }
        if grid[curr - size].shaded.is_none() {
            grid[curr - size].shaded = Some(false);
        }
    }

    if curr < size * (size - 1) {
        if curr % size != 0 && grid[curr + size - 1].shaded.is_none() {
            grid[curr + size - 1].shaded = Some(false);
        }
        if curr % size != size - 1 && grid[curr + size + 1].shaded.is_none() {
            grid[curr + size + 1].shaded = Some(false);
        }
        if grid[curr + size].shaded.is_none() {
            grid[curr + size].shaded = Some(false);
        }
    }

    if curr % size != 0 && grid[curr - 1].shaded.is_none() {
        grid[curr - 1].shaded = Some(false);
    }
    if curr % size != size - 1 && grid[curr + 1].shaded.is_none() {
        grid[curr + 1].shaded = Some(false);
    }
    if grid[curr].shaded.is_none() {
        grid[curr].shaded = Some(false);
    }
}

pub fn solve(mut grid: Vec<Cell>, size: usize) -> (Vec<Cell>, bool, i32) {
    let mut solved = false;
    let mut loop_count = 1;
    while !solved && loop_count < 101 {
        loop_count += 1;
        for ind in 0..size * size {
            let curr: &Cell = &grid[ind];
            if let Some(i) = curr.clue {
                if count_empty_neighbours(&grid, size, ind)
                    == i - count_shaded_neighbours(&grid, size, ind)
                {
                    turn_on_empty_neighbours(&mut grid, size, ind);
                }
                if count_shaded_neighbours(&grid, size, ind) == i {
                    turn_off_empty_neighbours(&mut grid, size, ind);
                }
            }
        }
        solved = true;
        for c in &grid {
            if c.shaded.is_none() {
                solved = false;
                continue;
            }
        }
    }
    (grid, solved, loop_count)
}

pub fn read_task(task: &str, size: usize) -> Vec<Cell> {
    let mut grid: Vec<Cell> = Vec::with_capacity(size * size);
    for c in task.chars() {
        // 0-9, a-z, A-Z
        // digits clues, letters spaces => letter - 9 gives corresponding number of spaces
        // e.g. a = 10 -> 1 space
        if let Some(number) = c.to_digit(36) {
            match number {
                digit if number < 10 => grid.push(Cell {
                    shaded: None,
                    clue: Some(digit.try_into().unwrap()),
                }),
                space => {
                    for _ in 0..(space - 9) {
                        grid.push(Cell {
                            shaded: None,
                            clue: None,
                        });
                    }
                }
            }
        }
    }
    return grid;
}

pub fn parse_answer(grid: &Vec<Cell>) -> String {
    let mut result = String::new();

    for i in grid {
        match i.shaded {
            Some(true) => result += "y",
            Some(false) => result += "n",
            None => println!("Something's fucked up"),
        }
    }

    return result;
}

pub fn print_clues(grid: &Vec<Cell>, size: usize) {
    println!("Clue grid:\n{}", clues_string(grid, size));
}

fn clues_string(grid: &Vec<Cell>, size: usize) -> String {
    let mut result = String::new();
    let mut col = 0;
    for i in grid.iter() {
        if let Some(j) = i.clue {
            result += &format!("{}", j);
        } else {
            result.push('.');
        }
        col += 1;
        if col == size {
            result.push('\n');
            col = 0;
        }
    }
    return result;
}

pub fn print_result(grid: &Vec<Cell>, size: usize) {
    println!("Result:\n{}", result_string(grid, size));
}

fn result_string(grid: &Vec<Cell>, size: usize) -> String {
    let mut result = "".to_string();
    let mut col = 0;
    for i in grid.iter() {
        match i.shaded {
            Some(true) => result.push('O'),
            Some(false) => result.push('x'),
            None => result.push('.'),
        }
        col += 1;
        if col == size {
            result.push('\n');
            col = 0;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn read_test() {
        let task = "a2d3b1b45a4e3a3a124b3a66c5534a7a5354a2a78b67a7b8b56777a5a7b987a46a7d5c5b4c1a";
        let size = 10;
        assert_eq!(read_task(task, size).len(), size * size);
    }

    #[test]
    fn puzzle1() {
        let task = "a2d3b1b45a4e3a3a124b3a66c5534a7a5354a2a78b67a7b8b56777a5a7b987a46a7d5c5b4c1a";
        let (grid, _solved, _loop_count) = solve(read_task(&task, 10), 10);
        let result = "OxOOOOOOxx
xxxxxxxxOx
OxOOxxOxOO
OxxOxxxOxx
xOOOOOOOxO
OOOOxxxOxO
xOOxOOOOOO
OOOxOOOOOx
OOOOOOOxxO
OOxOxxOxxx
";
        assert_eq!(result, result_string(&grid, 10));
    }

    #[test]
    fn puzzle2() {
        let task = "b4f34c86a4b4a99a7544b688a765b13a7a6a5a202b3c4b24a32a54a2a322b2a0f43b12122d1";
        let (grid, _solved, _loop_count) = solve(read_task(&task, 10), 10);
        let result = "OxxOOOxxOO
xOOOOxxxxO
OOOOOOOOOO
xOOOOOOxxx
xxOOxxOxxO
xxxOOxOOOx
xxOxxxxOxx
xOxOxxxOxx
OxxxxOxxxx
xxOxOxOOxO
";
        assert_eq!(result, result_string(&grid, 10));
    }
}
