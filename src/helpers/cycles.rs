use super::Grid;

#[derive(Debug, Clone, PartialEq)]
pub struct Cycleable<T: Eq> {
    prevs: Vec<Grid<T>>,
    /// start, end
    pub cycles_at: Option<(usize, usize)>,
}

impl<T: Eq> Cycleable<T> {
    pub fn new() -> Self {
        Self {
            prevs: Vec::new(),
            cycles_at: None,
        }
    }

    pub fn add(&mut self, grid: Grid<T>) {
        if let Some(i) = self.prevs.iter().position(|g| g == &grid) {
            self.cycles_at = Some((i, self.prevs.len()));
        }

        self.prevs.push(grid);
    }

    pub fn at_cycled(&self, time: usize) -> Option<&Grid<T>> {
        let (c_start, c_end) = self.cycles_at?;

        let i = (time - c_start) % (c_end - c_start) + c_start;

        self.prevs.get(i)
    }

    pub fn cycle_found(&self) -> bool {
        self.cycles_at.is_some()
    }
}
