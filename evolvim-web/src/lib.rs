use wasm_bindgen::prelude::*;
use lib_evolvim as evolvim;
use evolvim::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Universe {
    board: Board,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        Universe { board: Board::default() }
    }

    pub fn width(&self) -> usize {
        self.board.get_board_width()
    }

    pub fn height(&self) -> usize {
        self.board.get_board_height()
    }

    pub fn update(&mut self) {
        self.board.update(10.0);
    }

    pub fn season(&self) -> String {
        self.board.get_season()
    }

    pub fn time(&self) -> f64 {
        self.board.get_time()
    }

    pub fn count_creatures(&self) -> usize {
        self.board.creatures.len()
    }
}
