#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        Self {
            init_position: init_position,
            init_velocity: init_velocity,
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;
    fn next(&mut self) -> Option<ThrowObject> {
        let dt = 1.0;
        let g = 9.8;
        let p_x = self.actual_velocity.x + self.actual_position.x * dt; // p_x = v_x + x * dt
        let p_y = self.actual_position.y + self.actual_velocity.y * dt - 0.5 * g * dt.powi(2); // p_y = y + v_y * dt - (1/2) * g * dt²
        let v_x = self.actual_velocity.x;
        let v_y = self.actual_velocity.y - g * dt; // v_y = v_y - g * dt
        self.time += dt;
        self.actual_position = Object {
            x: ((p_x * 10.0).round()) / 10.0,
            y: ((p_y * 10.0).round()) / 10.0,
        };
        self.actual_velocity = Object {
            x: ((v_x * 10.0).round()) / 10.0,
            y: ((v_y * 10.0).round()) / 10.0,
        };

        if self.actual_position.y < 0.0 {
            return None;
        }

        Some(ThrowObject {
            init_position: self.init_position,
            init_velocity: self.init_velocity,
            actual_position: self.actual_position,
            actual_velocity: self.actual_velocity,
            time: self.time,
        })
    }
}
