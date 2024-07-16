use super::road::Lane;

#[derive(PartialEq, Eq, Clone)]
pub struct TrafficLight {
    x: i32,
    y: i32,
    pub color: State,
    pub lane: Lane,
    // pub last_change: std::time::Instant,
}

#[derive(PartialEq, Eq, Clone)]
pub enum State {
    Red,
    Green,
}

impl TrafficLight {
    pub fn new(x: i32, y: i32, lane: Lane) -> TrafficLight {
        TrafficLight {
            x,
            y,
            color: State::Red,
            lane,
            // last_change: std::time::Instant::now(),
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::WindowCanvas) {
        let color = match self.color {
            State::Red => sdl2::pixels::Color::RGB(255, 0, 0),
            State::Green => sdl2::pixels::Color::RGB(0, 255, 0),
        };

        canvas.set_draw_color(color);
        canvas
            .fill_rect(sdl2::rect::Rect::new(self.x as i32, self.y as i32, 20, 20))
            .unwrap();
    }

    pub fn change_state(&mut self) {
        self.color = match self.color {
            State::Red => State::Green,
            State::Green => State::Red,
        };
        // self.last_change = std::time::Instant::now();
    }
    pub fn set_state(&mut self, state: State) {
        self.color = state;
    }
}
