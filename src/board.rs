use rand::Rng;

const CENTER_FULL: &str = "██";
const CENTER_EMPTY: &str = "  ";

#[derive(Debug)]
pub struct Board {
    /// current state field
    current: Vec<Vec<bool>>,
    /// future state field
    future: Vec<Vec<bool>>,
    /// field width
    cols: usize,
    /// field height
    rows: usize,
}

impl Board {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            current: vec![vec![false; rows]; cols],
            future: vec![vec![false; rows]; cols],
            cols,
            rows,
        }
    }

    pub fn resize(&mut self, new_cols: usize, new_rows: usize) {
        self.current.resize(new_cols, vec![false; new_rows]);
        for col in 0..new_cols {
            self.current[col].resize(new_rows, false);
        }
    }

    pub fn populate(&mut self, col: usize, row: usize) {
        if col < self.cols && row < self.rows {
            self.current[col][row] = true;
        }
    }

    pub fn depopulate(&mut self, col: usize, row: usize) {
        if col < self.cols && row < self.rows {
            self.current[col][row] = false;
        }
    }

    pub fn populate_random(&mut self) {
        let size = self.cols * self.rows / 4;
        for _ in 0..size {
            let x = rand::thread_rng().gen_range(0..self.cols);
            let y = rand::thread_rng().gen_range(0..self.rows);
            self.current[x][y] = true;
        }
    }

    fn alive(&self, x: i32, y: i32) -> bool {
        let w = self.cols as i32;
        let h = self.rows as i32;
        let i = ((x + w) % w) as usize;
        let j = ((y + h) % h) as usize;
        self.current[i][j]
    }

    fn next(&self, x: i32, y: i32) -> bool {
        let mut alive = 0;
        for j in y - 1..y + 2 {
            for i in x - 1..x + 2 {
                if (j != y || i != x) && self.alive(i, j) {
                    alive += 1;
                }
            }
        }
        alive == 3 || (alive == 2 && self.alive(x, y))
    }

    pub fn step(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                self.future[x][y] = self.next(x as i32, y as i32);
            }
        }
        std::mem::swap(&mut self.current, &mut self.future);
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_string();
        for y in 0..self.rows {
            for x in 0..self.cols {
                if self.current[x][y] {
                    s += CENTER_FULL;
                } else {
                    s += CENTER_EMPTY;
                }
            }
            s += "\n";
        }
        s
    }
}
