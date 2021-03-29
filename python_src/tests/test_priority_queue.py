from .context import priority_queue
import unittest

class TestPriorityQueue(unittest.TestCase):
    def test_push_overflow(self):
        pq = priority_queue.PriorityQueue(3)
        item2 = priority_queue.PriorityQueueItem(1, "toto")
        try:
            for _ in range(4):
                pq.push(item2)
            raise Exception("Failed test")
        except BufferError as e:
            self.assertEqual(str(e), "Max size reached")

    def test_peek(self):
        pq = priority_queue.PriorityQueue(2)
        item1 = priority_queue.PriorityQueueItem(5, "zozo")
        item2 = priority_queue.PriorityQueueItem(1, "toto")
        pq.push(item1)
        pq.push(item2)
        self.assertEqual(pq.peek().value, "toto")

    def test_pop(self):
        pq = priority_queue.PriorityQueue(5)
        item1 = priority_queue.PriorityQueueItem(5, "zozo")
        item2 = priority_queue.PriorityQueueItem(1, "toto")
        item3 = priority_queue.PriorityQueueItem(2, "bobo")
        item4 = priority_queue.PriorityQueueItem(4, "bozo")
        item5 = priority_queue.PriorityQueueItem(3, "nono")
        pq.push(item1)
        pq.push(item2)
        pq.push(item3)
        pq.push(item4)
        pq.push(item5)
        el1 = pq.pop()
        self.assertEqual(el1.value, "toto")
        el2 = pq.pop()
        self.assertEqual(el2.value, "bobo")
        el3 = pq.pop()
        self.assertEqual(el3.value, "nono")
        el4 = pq.pop()
        self.assertEqual(el4.value, "bozo")
        el5 = pq.pop()
        self.assertEqual(el5.value, "zozo")

    def test_empty_pop(self):
        pq = priority_queue.PriorityQueue(5)
        try:
            pq.pop()
        except IndexError as e:
            self.assertEqual(str(e), "Empty queue")

if __name__ == "__main__":
    unittest.main()

