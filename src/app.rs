use crate::board;

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// current epoch
    pub epoch: usize,

    pub board: board::Board,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cols: usize, rows: usize) -> Self {
        println!("create cols: {}, rows: {}", cols, rows);
        let board = board::Board::new(cols, rows);
        // board.populate_random();
        Self {
            should_quit: false,
            epoch: 0,
            board,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_epoch(&mut self) {
        if let Some(res) = self.epoch.checked_add(1) {
            self.epoch = res;
            self.board.step();
        }
    }

    pub fn populate_board(&mut self, col: usize, row: usize) {
        self.board.populate(col, row);
    }

    pub fn depopulate_board(&mut self, col: usize, row: usize) {
        self.board.depopulate(col, row);
    }

    pub fn resize_board(&mut self, new_cols: usize, new_rows: usize) {
        self.board.resize(new_cols, new_rows);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_epoch() {
        let mut app = App::new(4, 4);
        app.increment_epoch();
        assert_eq!(app.epoch, 1);
    }
}
