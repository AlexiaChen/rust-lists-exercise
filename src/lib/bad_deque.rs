use std::cell::RefCell;
use std::rc::Rc;

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
}

type NodeRcCell<T> = Rc<RefCell<Node<T>>>;
type Link<T> = Option<NodeRcCell<T>>;

struct Node<T> {
    element: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Deque<T> {
    /// create new empty deque
    /// ### Example
    /// ```rust
    /// use mylist::bad_deque::Deque;
    /// let deque = Deque::<i32>::new();
    /// ```
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
        }
    }

    /// push an element in front of deque
    /// ### Example
    /// ```rust
    /// use mylist::bad_deque::Deque;
    /// let mut deque = Deque::new();
    /// deque.push_front(3);
    /// ```
    pub fn push_front(&mut self, element: T) {
        let new_node = Node::new(element);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    /// pop an element in front of deque
    /// ### Example
    /// ```rust
    /// use mylist::bad_deque::Deque;
    /// let mut deque = Deque::new();
    /// deque.push_front(3);
    /// assert_eq!(deque.pop_front(), Some(3));
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head_node| {
            match head_node.borrow_mut().next.take() {
                Some(next_node) => {
                    next_node.borrow_mut().prev.take();
                    self.head = Some(next_node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(head_node).ok().unwrap().into_inner().element
        })
    }
}

impl<T> Node<T> {
    fn new(element: T) -> NodeRcCell<T> {
        Rc::new(RefCell::new(Node {
            element: element,
            prev: None,
            next: None,
        }))
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::Deque;

    #[test]
    fn basics() {
        let mut deque = Deque::new();
        assert_eq!(deque.pop_front(), None);

        deque.push_front(3);
        deque.push_front(4);
        deque.push_front(5);

        assert_eq!(deque.pop_front(), Some(5));
        assert_eq!(deque.pop_front(), Some(4));
        deque.push_front(1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), None);
    }
}
