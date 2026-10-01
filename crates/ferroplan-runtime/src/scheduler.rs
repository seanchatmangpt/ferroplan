use std::collections::VecDeque;

#[derive(Default)]
pub struct Scheduler<T> {
    q: VecDeque<T>,
}

impl<T> Scheduler<T> {
    pub fn push(&mut self, x: T) {
        self.q.push_back(x)
    }

    pub fn pop_next(&mut self) -> Option<T> {
        self.q.pop_front()
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.q.is_empty()
    }
}
