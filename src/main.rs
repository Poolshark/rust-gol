use rand::Rng;

#[derive(PartialEq)]
pub enum Seed {
    Random,
    Glider,
}

#[derive(PartialEq, Clone)]
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

fn check(x: &usize, y: &usize) -> Vec<usize> {
    let mut coord: Vec<usize> = vec![0; 0];

    if *x < 0 {
        coord[0] = WIDTH + x;
    } else if *x > WIDTH - 1 {
        coord[0] = x - WIDTH - 1;
    }

    if *y < 0 {
        coord[1] = HEIGHT + y;
    } else if *y > HEIGHT - 1 {
        coord[1] = y - WIDTH - 1;
    }

    return coord;
}

fn update_cell(x: &usize, y: &usize) -> i8 {
    todo!();
}

fn update_grid(grid: &mut Grid) -> () {
    let mut new_grid = grid.clone();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            todo!();
        }
    }

    *grid = new_grid;
}

fn main() {
    let mut grid: Grid = vec![vec![CellState::Dead; WIDTH]; HEIGHT];

    create_grid(&mut grid, None);

    print_grid(&grid)
}
