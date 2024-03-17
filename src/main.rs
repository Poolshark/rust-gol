use console::Term;
use rand::Rng;
use std::thread;

#[derive(PartialEq)]
pub enum Seed {
    Random,
    Glider,
    Blinker,
    Test,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CellState {
    Dead = 0,
    Alive = 1,
}

type Grid = Vec<Cell>;

type Cell = Vec<CellState>;

const WIDTH: usize = 50;
const HEIGHT: usize = 25;

fn print_grid(grid: &Grid) -> () {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if grid[i][j] == CellState::Dead {
                print!(" ");
            } else {
                print!("*");
            }
        }
        println!();
    }
}

fn set_initial(grid: &mut Grid, seed: Seed) -> () {
    if seed == Seed::Glider {
        grid[0][1] = CellState::Alive;
        grid[1][2] = CellState::Alive;
        grid[2][0] = CellState::Alive;
        grid[2][1] = CellState::Alive;
        grid[2][2] = CellState::Alive;
    } else if seed == Seed::Test {
        grid[0][0] = CellState::Alive;
    } else if seed == Seed::Blinker {
        grid[0][1] = CellState::Alive;
        grid[1][1] = CellState::Alive;
        grid[2][1] = CellState::Alive;
    }
}

fn create_grid(grid: &mut Grid, seed: Option<Seed>) -> () {
    let which = seed.unwrap_or(Seed::Random);
    let mut rng = rand::thread_rng();

    if which == Seed::Random {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                grid[i][j] = if rng.gen_range(0..2) == 1 {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
            }
        }
    } else {
        set_initial(grid, which);
    }
}

fn get_neighbours(grid: &Grid, x: &usize, y: &usize) -> i8 {
    #[derive(PartialEq)]
    enum Axis {
        X,
        Y,
    }
    fn fix_bounds(coord: i8, axis: Axis) -> usize {
        let mut fix: usize = coord as usize;

        let w: i8 = WIDTH as i8;
        let h: i8 = HEIGHT as i8;

        if axis == Axis::X {
            if coord < 0 {
                fix = (w + coord) as usize;
            } else if coord > w - 1 {
                fix = (coord - w) as usize;
            }
        } else if axis == Axis::Y {
            if coord < 0 {
                fix = (h + coord) as usize;
            } else if coord > h - 1 {
                fix = (coord - h) as usize;
            }
        }

        return fix;
    }

    let _x = *x as i8;
    let _y = *y as i8;

    let mut fy: usize;
    let mut fx: usize;

    let mut sum_alive: i8 = 0;

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            } else {
                fy = fix_bounds(_y + i, Axis::Y);
                fx = fix_bounds(_x + j, Axis::X);
                if grid[fy][fx] == CellState::Alive {
                    sum_alive += 1;
                }
            }
        }
    }

    return sum_alive;
}

fn update_cell(old_grid: &Grid, new_grid: &mut Grid, x: &usize, y: &usize) -> () {
    let sum = get_neighbours(&old_grid, &x, &y);
    let curr_state = old_grid[*y][*x];

    // GOL rules
    if curr_state == CellState::Dead {
        if sum == 3 {
            new_grid[*y][*x] = CellState::Alive;
        }
    } else {
        if sum < 2 || sum > 3 {
            new_grid[*y][*x] = CellState::Dead;
        }
    }
}

fn update_grid(grid: &mut Grid) -> () {
    let mut new_grid = grid.clone();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            update_cell(&grid, &mut new_grid, &j, &i);
        }
    }

    *grid = new_grid;
}

fn run(time: u64, intervall: u64, grid: &mut Grid) -> () {
    let term = Term::stdout();
    let mut timer = intervall;
    print_grid(&grid);
    thread::sleep(std::time::Duration::from_millis(intervall));
    while timer <= time {
        update_grid(grid);
        term.move_cursor_up(HEIGHT).unwrap();
        print_grid(grid);
        thread::sleep(std::time::Duration::from_millis(intervall));
        term.move_cursor_up(HEIGHT).unwrap();
        timer += intervall;
    }
}

fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    let mut grid: Grid = vec![vec![CellState::Dead; WIDTH]; HEIGHT];
    create_grid(&mut grid, Some(Seed::Random));

    run(10000, 100, &mut grid);
}
