use std::cell::RefCell;
use std::rc::Rc;

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
}

type NodeRcCell<T> = Rc<RefCell<Node<T>>>;
type Link<T> = Option<NodeRcCell<T>>;

pub struct Node<T> {
    element: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
        }
    }

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
