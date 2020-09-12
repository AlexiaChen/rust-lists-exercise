use std::rc::Rc;

// list1 -> A ---+
//               |
//               v
// list2 ------> B -> C -> D
//               ^
//               |
// list3 -> X ---+
// persistent stack use shared ownership
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, element: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                element: element,
                next: self.head.clone()
            }))
        }
    }
}
