pub struct Tabs<T> {
    index: isize,
    positions: Vec<(u16, u16, T)>,
}

impl<T> Tabs<T> {
    // Constructor
    pub fn new(positions: Vec<(u16, u16, T)>) -> Self {
        Tabs {
            positions,
            index: 0,
        }
    }

    // Position of current tab
    pub fn position(&self) -> (u16, u16) {
        (
            self.positions[self.index as usize].0,
            self.positions[self.index as usize].1,
        )
    }

    // Value of current tab
    pub fn value(&self) -> &T {
        &self.positions[self.index as usize].2
    }

    // Increment index of tab
    // ! Running twice for some reason and incrementing index more than required
    pub fn next(&mut self) {
        let len = self.positions.len() as isize;
        self.index = (self.index + 1) % len;
    }

    // Decrement index of tab
    pub fn prev(&mut self) {
        let len = self.positions.len() as isize;
        self.index = (self.index - 1 + len) % len;
    }
}
