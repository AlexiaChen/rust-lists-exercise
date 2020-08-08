pub mod ok_stack {
    pub struct Stack {
        head: Link,
    }

    type Link = Option<Box<Node>>;
    
    struct Node {
        element: i32,
        next: Link,
    }
    
    impl Stack {
        pub fn new() -> Self {
            Stack {
                head: None
            }
        }
    
        pub fn push(&mut self, element: i32) {
            let new_node = Box::new(Node {
                element: element,
                next: self.head.take()
            });
    
            self.head = Some(new_node);
    
        }
    
        pub fn pop(&mut self) -> Option<i32> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.element
            })
        }
    }
    
    
    impl Drop for Stack {
        fn drop(&mut self) {
            let mut cur_link = self.head.take();
            while let Some(mut node) = cur_link {
                cur_link = node.next.take();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn stack_tests() {
        use super::ok_stack::Stack;
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