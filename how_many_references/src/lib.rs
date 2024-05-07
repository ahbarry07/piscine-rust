pub use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Node {
        Node { ref_list: ref_list }
    }
    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element)
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        self.ref_list.retain(|nod|{
            (*nod != element && Rc::strong_count(&nod) == Rc::strong_count(&element)) || (*nod == element && Rc::strong_count(&nod) != Rc::strong_count(&element))
        })    
    }

}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
