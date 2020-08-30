pub struct Stack<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    /// single instance creator
    /// ### Example
    /// ```rust
    /// use mylist::ok_stack::Stack;
    /// let stack = Stack::<i32>::new();
    /// ```
    pub fn new() -> Self {
        Stack { head: None }
    }

    /// push new element to stack
    /// ### Example
    /// ```rust
    /// use mylist::ok_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// ```
    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element: element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    /// pop element in top of stack
    /// ### Example
    /// ```rust
    /// use mylist::ok_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// assert_eq!(stack.pop(), Some(5));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.element
        })
    }

    /// peek a element in top of stack
    /// ### Example
    /// ```rust
    /// use mylist::ok_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// assert_eq!(stack.peek(), Some(&5));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    /// peek a mutable element in top of stack and you can change it
    /// ### Example
    /// ```rust
    /// use mylist::ok_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// assert_eq!(stack.peek(), Some(&5));
    /// stack.peek_mut().map(|value| *value = 3);
    /// assert_eq!(stack.peek(), Some(&3));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }

    // declare a fresh lifetime here for the *exact* borrow that
    // creates the iter. Now &self needs to be valid as long as the
    // Iter is around.
    /// return a iter point to head
    /// ### Example
    /// ```rust
    /// use mylist::ok_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// let mut iter = stack.iter();
    /// assert_eq!(iter.next(), Some(&5));
    /// ```
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

   
    /// return a mutable iter point to head
    /// ### Example
    /// ```rust
    /// use mylist::ok_stack::Stack;
    /// let mut stack = Stack::new();
    /// stack.push(5);
    /// let mut iter = stack.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 5));
    /// ```
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

// Impl Drop trait for Stack<T>
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut node) = cur_link {
            cur_link = node.next.take();
        }
    }
}

// Iter is generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// Impl Iterator trait for Iter<T>
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.element
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.element
        })
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

        stack.push(15);
        stack.peek_mut().map(|value| *value = 11);
        assert_eq!(stack.peek(), Some(&11));
    }

    #[test]
    fn iter_tests() {
        use super::Stack;
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        let mut iter_mut = stack.iter_mut();
        iter_mut.next().map(|value| *value = 11);
        iter_mut.next().map(|value| *value = 12);
        iter_mut.next().map(|value| *value = 13);
        iter_mut = stack.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 11));
        assert_eq!(iter_mut.next(), Some(&mut 12));
        assert_eq!(iter_mut.next(), Some(&mut 13));
    }
}
