#![feature(arc_new_cyclic)]

use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Foo {
    name: String,
    parent: Weak<RefCell<Self>>,
    children: Vec<Rc<RefCell<Self>>>,
}

impl Foo {
    fn new(name: String) -> Self {
        Foo {
            name,
            parent: Weak::new(),
            children: vec![],
        }
    }

    fn dump(&self, level: usize) {
        for _ in 0..level {
            print!("  ");
        }

        /*
        let parent_name = match self.parent.upgrade() {
            Some(p) => &p.borrow().name,
            None => "none"
        };
         */
        let parent_name = self.parent.upgrade()
            .as_ref()
            .map(|p| p.borrow().name.clone())
            .unwrap_or_else(|| "none".to_string());
        
        println!("{} [parent: {}]", self.name, parent_name);
        for c in &self.children {
            c.borrow().dump(level+1);
        }
    }
}

fn main() {
    let child1 = Rc::new(RefCell::new(Foo::new("child1".to_owned())));
    let child2 = Rc::new(RefCell::new(Foo::new("child2".to_owned())));
    let parent = Rc::new(RefCell::new(Foo::new("parent".to_owned())));
    {
        
        let kids = &mut parent.borrow_mut().children;
        child1.borrow_mut().parent = Rc::downgrade(&parent);
        child2.borrow_mut().parent = Rc::downgrade(&parent);
        kids.push(child1);
        kids.push(child2);
    }

    parent.borrow().dump(0);
    
}
