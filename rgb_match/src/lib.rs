#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut components = [&mut self.r, &mut self.g, &mut self.b, &mut self.a];

        for component in &mut components {
            if **component == first {
                **component = second;
            } else if **component == second {
                **component = first;
            }
        }

        self
    }
}
