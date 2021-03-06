
pub struct Stack {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}
struct Node {
    element: i32,
    next: Link,
}
impl Stack {
    /// single instance creator
    /// ### Example
    /// ```rust
    /// use mylist::bad_stack::Stack;
    /// let stack = Stack::new();
    /// ```
    pub fn new() -> Self {
        Stack { head: Link::Empty }
    }

    /// push new element to stack
    /// ### Example
    /// ```rust
    /// use mylist::bad_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// ```
    pub fn push(&mut self, element: i32) {
        let new_node = Box::new(Node {
            element: element,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    /// pop element in top of stack
    /// ### Example
    /// ```rust
    /// use mylist::bad_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// assert_eq!(stack.pop(), Some(5));
    /// ```
    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }
}
impl Drop for Stack {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = cur_link {
            cur_link = std::mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn stack_tests() {
        use super::Stack;
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
}
