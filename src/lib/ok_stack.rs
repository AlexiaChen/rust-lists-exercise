pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }
}
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn stack_tests() {
        use super::Stack;

        {
            let mut new_stack = Stack::new();

            assert_eq!(new_stack.pop(), None);

            new_stack.push(1);
            new_stack.push(2);
            new_stack.push(3);

            assert_eq!(new_stack.pop(), Some(3));
            assert_eq!(new_stack.pop(), Some(2));

            new_stack.push(4);
            new_stack.push(5);

            assert_eq!(new_stack.pop(), Some(5));
            assert_eq!(new_stack.pop(), Some(4));

            assert_eq!(new_stack.pop(), Some(1));
            assert_eq!(new_stack.pop(), None);
        }

        {
            let mut new_stack = Stack::new();

            assert_eq!(new_stack.pop(), None);

            new_stack.push("world");
            new_stack.push("hello");

            assert_eq!(new_stack.pop(), Some("hello"));
            assert_eq!(new_stack.pop(), Some("world"));

            assert_eq!(new_stack.pop(), None);
        }
    }

    #[test]
    fn peek_tests() {
        use super::Stack;
        let mut stack = Stack::new();
        stack.push(5);
        stack.push(4);
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        stack.pop();
        assert_eq!(stack.peek(), Some(&4));
        assert_eq!(stack.peek(), Some(&4));
        stack.pop();
        stack.pop();
        assert_eq!(stack.peek(), None);
    }
}
