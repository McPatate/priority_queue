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
    fn heapify_down(&mut self, pos: usize) {
        let child_pos = 2 * (pos + 1);
        let mut smallest_child = child_pos - 1;
        if child_pos > self.queue.len() {
            return;
        } else if child_pos < self.queue.len() {
            smallest_child = if self.queue[child_pos - 1].0 < self.queue[child_pos].0 {
                child_pos - 1
            } else {
                child_pos
            };
        }
        if self.queue[smallest_child].0 < self.queue[pos].0 {
            self.queue.swap(pos, smallest_child);
            self.heapify_down(smallest_child);
        }
    }
}

impl<T> PriorityQueue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            max_size: size,
            queue: Vec::with_capacity(size),
        }
    }
    pub fn push(&mut self, priority: usize, value: T) -> Result<(), MaxSizeReachedError> {
        if self.queue.len() == self.max_size {
            return Err(MaxSizeReachedError);
        }
        self.queue.push((priority, value));
        self.heapify_up(self.queue.len() - 1);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        self.pop_at(0)
    }

    pub fn pop_at(&mut self, idx: usize) -> Option<(usize, T)> {
        if self.queue.len() == 0 {
            return None;
        }
        let last_pos = self.queue.len() - 1;
        self.queue.swap(idx, last_pos);
        let ret = self.queue.pop();
        self.heapify_up(idx);
        self.heapify_down(idx);
        ret
    }

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
        pq.push(5, "hello");
        pq.push(1, "bozo");
        let bozo = pq.peek().unwrap();
        assert_eq!(*bozo, "bozo");
    }

    #[test]
    fn test_pop() {
        let mut pq: PriorityQueue<&str> = PriorityQueue::new(3);
        pq.push(5, "hello");
        pq.push(1, "bozo");
        pq.push(2, "bono");
        let el1 = pq.pop().unwrap();
        assert_eq!(el1, (1, "bozo"));
        let el2 = pq.pop().unwrap();
        assert_eq!(el2, (2, "bono"));
        let el3 = pq.pop().unwrap();
        assert_eq!(el3, (5, "hello"));
    }
}
