extern crate fixedbitset;
extern crate web_sys;
use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;
use std::fmt;

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        // Read the vector as a 2d array
        return (row * self.width + column) as usize;
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        // Count the number of adjacent live neighbors
        // The universe will wrap at the edges
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        return count;
    }

    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &fixedbitset::FixedBitSet {
        return &self.cells;
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next.set(idx, match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise
                });
            }
        }

        self.cells = next;
    }

    pub fn new(_width: Option<u32>, _height: Option<u32>) -> Universe {
        utils::set_panic_hook();

        log!("New Universe ({}, {})", _width.unwrap_or(64), _height.unwrap_or(64));

        let width = _width.unwrap_or(64);
        let height = _height.unwrap_or(64);

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        
        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        return Universe {
            width,
            height,
            cells
        };
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (self.width * self.height) as usize;

        self.cells = FixedBitSet::with_capacity(size);
        self.cells.set_range(.., false);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * self.height) as usize;

        self.cells = FixedBitSet::with_capacity(size);
        self.cells.set_range(.., false);
    }

    pub fn clear(&mut self) {
        self.cells.set_range(.., false);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn render(&self) -> String {
        return self.to_string();
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn cells(&self) -> *const u32 {
        return self.cells.as_slice().as_ptr();
    }
}