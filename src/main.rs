use mylist::bad_stack::bad_stack::Stack;

pub fn main() {
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