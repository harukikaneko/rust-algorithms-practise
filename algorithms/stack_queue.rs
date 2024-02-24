#[derive(Debug, Clone)]
pub struct Stack {
    values: Vec<usize>,
    top: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            values: [].to_vec(),
            top: 0,
        }
    }

    pub fn push(&mut self, value: usize) {
        if !self.values.is_empty() {
            self.values.insert(self.top + 1, value);
            return self.top += 1;
        }

        self.values.insert(self.top, value);
    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        let target_index = self.values.len() - 1;
        let value = self.values.get(target_index).copied();
        self.values.remove(target_index);
        self.top = target_index;
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
}
