#[derive(Debug, Clone)]
pub struct Stack {
    values: Vec<usize>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            values: [].to_vec(),
        }
    }

    pub fn push(&mut self, value: usize) {
        let top = self.values.len();
        self.values.insert(top, value);
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let target_index = self.values.len() - 1;
        let value = self.values.get(target_index).copied();
        self.values.remove(target_index);
        value
    }

    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }
}

#[derive(Debug, Clone)]
pub struct Queue {
    values: Vec<usize>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            values: [].to_vec(),
        }
    }

    pub fn push_front(&mut self, value: usize) {
        self.values.insert(0, value);
    }

    pub fn push_back(&mut self, value: usize) {
        let tail = self.values.len();
        self.values.insert(tail, value);
    }

    pub fn pop_front(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let value = self.values.get(0).copied();
        self.values.remove(0);
        value
    }

    pub fn pop_back(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let tail = self.values.len() - 1;
        let value = self.values.get(tail).copied();
        self.values.remove(tail);
        value
    }

    pub fn is_empty(&self) -> bool {
        self.values.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_simple_lifo() {
        let mut queue = Vec::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_fifo() {
        let mut stack = VecDeque::new();

        stack.push_back(1);
        stack.push_back(2);
        stack.push_back(3);

        assert_eq!(stack.pop_front(), Some(1));
        assert_eq!(stack.pop_front(), Some(2));
        assert_eq!(stack.pop_front(), Some(3));
        assert_eq!(stack.pop_front(), None);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None)
    }

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();

        queue.push_front(1);
        queue.push_back(3);
        queue.push_front(4);
        queue.push_back(5);
        
        assert_eq!(queue.pop_front(), Some(4));
        assert_eq!(queue.pop_back(), Some(5));
        assert_eq!(queue.pop_front(), Some(1));
        assert_eq!(queue.pop_back(), Some(3));
        assert_eq!(queue.pop_front(), None);
        assert_eq!(queue.pop_back(), None);
    }
}
