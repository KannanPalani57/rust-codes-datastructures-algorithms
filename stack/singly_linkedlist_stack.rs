
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem: T, 
    next: Link<T>
}


impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem, 
            next: self.head.take()
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
		self.head = node.next;
		node.elem
	})
    }

    fn peak(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    fn peak_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);



impl<T> Iterator for IntoIter<T> {
    type Item = T; 

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}




fn main() {
    println!("Singly Linked List");

    let mut list = List::new();
	list.push(12);
	list.push(24);

    match list.peak() {
        Some(item) => println!("item {}", item),
        None => println!("Stack is Empty")
    }

    list.peak_mut().map(|node| {
        *node = 55
    });

    match list.peak() {
        Some(item) => println!("item {}", item),
        None => println!("Stack is Empty")
    }

    let mut iter = list.into_iter();

    
    match iter.next() {
        Some(item) => println!("Popped item {}", item),
        None => println!("Stack is Empty")
    }

    
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
