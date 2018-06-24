use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let _new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
    }
    
    pub fn pop(&mut self) -> Option<i32> {
        match self.head {
            Link::Empty => {
                //TODO
            }
            Link::More(ref _node) => {
                //TODO
            }
        };
        unimplemented!()
    }
}