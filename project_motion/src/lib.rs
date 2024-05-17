#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        // Calculate new position and velocity using the projectile motion equations
        let g = 9.8; // Gravity in m/s^2

        // Update time
        self.time += 1.0;

        // Calculate new position
        let new_pos_x = self.init_position.x + self.init_velocity.x * self.time;
        let new_pos_y =
            self.init_position.y + self.init_velocity.y * self.time - 0.5 * g * self.time.powi(2);

        // Calculate new velocity
        let new_vel_x = self.init_velocity.x;
        let new_vel_y = self.init_velocity.y - g * self.time;

        // Update the actual position and velocity
        self.actual_position = Object {
            x: new_pos_x,
            y: new_pos_y,
        };
        self.actual_velocity = Object {
            x: new_vel_x,
            y: new_vel_y,
        };

        // Check if the object has reached the floor
        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}
