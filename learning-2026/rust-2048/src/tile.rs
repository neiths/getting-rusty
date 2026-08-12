use piston_window::*;
use opengl_graphics::GlGraphics;
use number_renderer::NumberRenderer;

#[derive(Clone, PartialEq)]
pub enum TileState {
    TileStatic,
    /// (t, x, y, origin_x, origin_y)
    TileMoving(f64, f64, f64, i32, i32),
    /// (t, size)
    TileNew(f64, f64),
    /// (t, size)
    TileCombine(f64, f64),
}

#[derive(Clone)]
pub struct Tile<'a> {
    pub score: i32,
    pub tile_x: i32,
    pub tile_y: i32,
    pub status: TileState,

    settings: &'a Settings,
}