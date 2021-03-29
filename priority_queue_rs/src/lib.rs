use std::fmt;

#[derive(Clone, Debug)]
pub struct MaxSizeReachedError;

impl fmt::Display for MaxSizeReachedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "priority queue has reached max size")
    }
}

pub struct PriorityQueue<T> {
    max_size: usize,
    queue: Vec<(usize, T)>,
}

impl<T> PriorityQueue<T> {
    fn heapify_up(&mut self, pos: usize) {
        if pos > 0 {
            let parent_pos: usize = (pos - 1) / 2;
            if self.queue[pos].0 < self.queue[parent_pos].0 {
                self.queue.swap(parent_pos, pos);
                self.heapify_up(parent_pos);
            }
        }
    }
    fn heapify_down() {}
}

impl<T> PriorityQueue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            max_size: size,
            queue: Vec::with_capacity(size),
        }
    }
    pub fn push(&mut self, value: T, priority: usize) -> Result<(), MaxSizeReachedError> {
        if self.queue.len() == self.max_size {
            return Err(MaxSizeReachedError);
        }
        self.queue.push((priority, value));
        self.heapify_up(self.queue.len() - 1);
        Ok(())
    }

    pub fn pop(&self) -> (usize, T) {
        self.pop_at(0)
    }

    pub fn pop_at(&self, idx: usize) -> (usize, T) {}

    pub fn peek(&self) -> Option<&T> {
        if self.queue.len() > 0 {
            return Some(&self.queue[0].1);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_priority_queue() {
        let pq: PriorityQueue<usize> = PriorityQueue::new(0);
    }

    #[test]
    fn test_empty_peek() {
        let pq: PriorityQueue<usize> = PriorityQueue::new(0);
        let res = pq.peek();
        assert_eq!(res, None);
    }

    #[test]
    fn test_push() {
        let mut pq: PriorityQueue<&str> = PriorityQueue::new(2);
        pq.push("hello", 5);
        pq.push("bozo", 1);
        let bozo = pq.peek().unwrap();
        assert_eq!(*bozo, "bozo");
    }
}
