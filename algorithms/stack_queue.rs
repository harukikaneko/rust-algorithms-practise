#[cfg(test)]
mod tests {
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
}