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

    /// return an iter point to head
    /// ### Example
    /// ```rust
    /// use mylist::persistence_list::List;
    /// let list = List::new();
    /// let list = list.append(3).append(4);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&3));
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }
}

/// Impl Drop trait for List<T> that using Rerference Counting
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head_link = self.head.take();
        while let Some(node) = head_link {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head_link = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.element
        })
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

    #[test]
    fn iter_tests() {
        let list = List::new();
        let list = list.append(3).append(4);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}
