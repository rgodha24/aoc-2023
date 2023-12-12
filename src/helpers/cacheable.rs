use std::collections::HashMap;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct NoCache;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Cache(HashMap<(usize, usize, usize), usize>);

pub trait Cacheable: Default + PartialEq + std::fmt::Debug + Clone {
    fn get(&self, _: &(usize, usize, usize)) -> Option<usize> {
        None
    }
    fn insert(&mut self, _: (usize, usize, usize), _: usize) {}
}

impl Cacheable for NoCache {}
impl Cacheable for Cache {
    fn get(&self, key: &(usize, usize, usize)) -> Option<usize> {
        self.0.get(key).copied()
    }

    fn insert(&mut self, key: (usize, usize, usize), value: usize) {
        self.0.insert(key, value);
    }
}
