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
    
    /// single instance creator
    /// ### Example
    /// ```rust
    /// use mylist::persistence_list::List;
    /// let list = List::<i32>::new();
    /// ```
    pub fn new() -> Self {
        List { head: None }
    }

    /// append element to list head
    /// ### Example
    /// ```rust
    /// use mylist::persistence_list::List;
    /// let list = List::new();
    /// let list = list.append(5).append(4);
    /// assert_eq!(list.head(), Some(&4));
    /// ```
    pub fn append(&self, element: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                element: element,
                next: self.head.clone(),
            })),
        }
    }

    /// return rest of list except to head 
    /// ### Example
    /// ```rust
    /// use mylist::persistence_list::List;
    /// let list = List::new();
    /// let list = list.append(5).append(4).append(3);
    /// let list = list.tail();
    /// assert_eq!(list.head(), Some(&4));
    /// ```
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
    
    /// return head value of list
    /// ### Example
    /// ```rust
    /// use mylist::persistence_list::List;
    /// let list = List::<i32>::new();
    /// assert_eq!(list.head(), None);
    /// ```
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);

    }
}