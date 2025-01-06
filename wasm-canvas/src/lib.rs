use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::console;

macro_rules! log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

pub struct CoordinateSystem<'a> {
    location: Coord,
    // maybe the bounds too
    context: &'a CanvasRenderingContext2d
}

impl<'a> CoordinateSystem<'a> {
    pub fn new(location: Coord, context: &'a CanvasRenderingContext2d) -> Self {
        Self { location, context }
    }

    pub fn draw_rect(&self, c: Coord, w: f64, h: f64) {
        let c = c.to_cs(self.location);
        self.context.begin_path();
        self.context.rect(c.x, c.y, w, h);
        self.context.stroke();
    }

    pub fn draw_line(&self, from: Coord, to: Coord) {
        let from = from.to_cs(self.location);
        let to = to.to_cs(self.location);
        draw_line(from, to, self.context);
    }
}

fn draw_line(from: Coord, to: Coord, context: &CanvasRenderingContext2d) {
    context.begin_path();
    context.move_to(from.x, from.y);
    context.line_to(to.x, to.y);
    context.stroke();
}

// fn draw_rect(context: &CanvasRenderingContext2d) {
//     context.rect(x, y, w, h);
// }

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    x: f64,
    y: f64
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Self {
        Coord { x, y }
    }

    pub fn newi(x: i32, y: i32) -> Self {
        Self::new(x as f64, y as f64)
    }

    /// IMPORTANT
    /// 
    /// The transformation from the canvas coordinate system to our more natural maths coordinate system
    pub fn to_cs(self, cs_location: Coord) -> Self {
        Self::new(cs_location.x + self.x, cs_location.y - self.y)
    }
}

#[wasm_bindgen(start)]
fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    // canvas.set_width(300);
    // canvas.set_height(300);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_line_width(2.0);
    
    context.rect(0., 0., 200., 200.);
    context.stroke();

    let cs = CoordinateSystem::new(Coord::newi(100, 100), &context);

    cs.draw_line(Coord::newi(0, 0), Coord::newi(30, 10));

    cs.draw_rect(Coord::newi(0, 0), 30., -10.);

    log!("hi");

    context.rect(0., 0., 30., 10.);
    context.stroke();

    // draw_line(Coord::newi(0, 0), Coord::newi(30, 10), &context);


}



