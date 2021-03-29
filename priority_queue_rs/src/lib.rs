struct PriorityQueueItem<T> {
    priority: usize,
    value: T,
}

struct PriorityQueue<T> {
    max_size: usize,
    queue: Vec<PriorityQueueItem<T>>,
}

impl<T> PriorityQueue<T> {
    fn new(size: usize) -> Self {
        Self {
            max_size: size,
            queue: Vec::with_capacity(size),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declare_priority_queue() {
        let pq: PriorityQueue<usize> = PriorityQueue::new(0);
    }
}
