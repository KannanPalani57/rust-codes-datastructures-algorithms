
use std::mem;


pub struct List {
    head: Link
}

enum Link {
    Empty, 
    More(Box<Node>)
}

struct Node {
    elem: i32, 
    next: Link
}


impl List {
    fn new() -> Self {
        List { head: Link::Empty  }
    }

    fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem, 
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None, 
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

fn main() {
    println!("Singly Linked List");
}

#[cfg(test)]
mod test{
use super::List;

#[test]
fn it_works() {
    
	let mut list = List::new();
	
	assert_eq!(list.pop(), None);
	
	list.push(1);
	
	assert_eq!(list.pop(), Some(1));
	assert_eq!(list.pop(), None);
}
}
