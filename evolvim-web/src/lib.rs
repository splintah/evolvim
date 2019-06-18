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

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let board = Board::<Brain>::load_from_bytes(bytes).unwrap();
        
        Universe { board }
    }

    pub fn width(&self) -> usize {
        self.board.get_board_width()
    }

    pub fn height(&self) -> usize {
        self.board.get_board_height()
    }

    pub fn update(&mut self) {
        self.board.update(0.001);
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

    pub fn tile_colour_hue(&self, x: usize, y: usize) -> f32 {
        self.board.terrain.get_tile_at((x, y)).get_hsba_color()[0]
    }

    pub fn tile_colour_saturation(&self, x: usize, y: usize) -> f32 {
        self.board.terrain.get_tile_at((x, y)).get_hsba_color()[1]
    }

    pub fn tile_colour_brightness(&self, x: usize, y: usize) -> f32 {
        self.board.terrain.get_tile_at((x, y)).get_hsba_color()[2]
    }

    pub fn tile_colour_alpha(&self, x: usize, y: usize) -> f32 {
        self.board.terrain.get_tile_at((x, y)).get_hsba_color()[3]
    }
}
