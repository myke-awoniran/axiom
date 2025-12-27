use std::cell::RefCell;
use std::rc::Rc;

type SingleLink = Option<Rc<RefCell<Node>>>;

/* A (transaction) log is a great use case for a linked list: They often grow to unexpected sizes,
    and indexing is not required.

    NOTE::
    Implementation complexity:: While a linked list is often a very simple type in other
    languages, it harbors a surprising amount of challenges in Rust. This is mostly due to the
    borrowing and ownership concepts which require me to think about what goes where in great detail.

    From a performance perspective:: finding a particular item in the singly
    linked list requires looking at the entire list in the
    worst case, resulting in a runtime complexity of O(n), with n being the number of items in the list.

*/

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}

// use case of the linkedList, implementing a transaction Log
// With append and remove
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

impl TransactionLog {
    pub fn new_empty_transaction() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => old_tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }
        self.length += 1;
        self.tail = Some(new_node);
    }

    //usecase- database log replay
    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }
}
