use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option;

#[derive(Debug)]
struct DoublyLinkedList<T: Copy> {
    ends: Option<(Rc<RefCell<DLLNode<T>>>, Rc<RefCell<DLLNode<T>>>)>,
}

#[derive(Debug)]
struct DLLNode<T: Copy> {
    v: T,
    prev: Option<Rc<RefCell<DLLNode<T>>>>,
    next: Option<Rc<RefCell<DLLNode<T>>>>,
}

impl<T: Copy> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList{
            ends: None,
        }
    }

    fn append(&mut self, v: T) {
        let node = Rc::new(RefCell::new(DLLNode{v, prev: None, next: None}));
        if let Some((_, ref mut tail)) = self.ends {  // head becomes a &mut Rc<RefCell<DLLNode<T>>>
            let _ = node.borrow_mut().prev.insert(Rc::clone(tail));  // Less performant because requires an additional borrow check over creating a node in this context
            let _ = tail.borrow_mut().next.insert(Rc::clone(&node));
        } else {
            let _ = self.ends.insert((node.clone(), node.clone()));
        }
    }

    fn get(&self, index: usize) -> Option<T> {
        match self.ends {
            None => None,
            Some((ref head, _)) => {  // &Rc<RefCell<DLLNode<T>>>
                let curr = head.borrow();
                for _ in 0..index {
                    curr = curr.next.as_ref().unwrap().borrow();  // &DLLNode<T>
                }
                let borrowed = curr;
                Some(borrowed.v.clone())  // TODO: Look into Ref
            },
        }
    }
}

fn main() {
    let mut l = DoublyLinkedList::new();
    l.append(1);   // Add a node to the linked list, with value 1, then have l point to ?
    l.append(2);   // Add a new node with value 2
}
