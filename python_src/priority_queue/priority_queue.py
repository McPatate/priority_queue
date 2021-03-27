from typing import Any, List

class PriorityQueueItem():
    def __init__(self, priority: int, elt: Any):
        self.priority = priority
        self.value = elt

class PriorityQueue():
    def __init__(self, size):
        self.queue: List[PriorityQueueItem] = [None] * size
        self.length = 0
        self.max_size = size

    def _heapify_up(self, pos):
        if pos > 0:
            parent_pos = (pos - 1) // 2
            if self.queue[pos].priority < self.queue[parent_pos].priority:
                tmp = self.queue[pos]
                self.queue[pos] = self.queue[parent_pos]
                self.queue[parent_pos] = tmp
                self._heapify_up(parent_pos)

    def _heapify_down(self, pos):
        child_pos = 2 * (pos + 1)
        smallest_child = child_pos - 1
        if child_pos > self.length:
            return
        elif child_pos < self.length:
            smallest_child = child_pos - 1 if self.queue[child_pos - 1].priority < self.queue[child_pos].priority else child_pos
        if self.queue[smallest_child].priority < self.queue[pos].priority:
            tmp = self.queue[smallest_child]
            self.queue[smallest_child] = self.queue[pos]
            self.queue[pos] = tmp
            self._heapify_down(smallest_child)

    def push(self, item: PriorityQueueItem):
        if self.length == self.max_size:
            raise BufferError("Max size reached")
        self.queue[self.length] = item
        self._heapify_up(self.length)
        self.length += 1

    def pop(self, pos = 0):
        if self.length == 0:
            raise Exception("Empty queue")
        ret = self.queue[pos]
        self.queue[pos] = self.queue[self.length - 1]
        self.queue[self.length - 1] = None
        self.length -= 1
        self._heapify_up(pos)
        self._heapify_down(pos)
        return ret

    def peek_min(self):
        return self.queue[0]

