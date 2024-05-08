#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let curr = self.head.take();
        self.head = Some({
            Node {
                value, 
                next: if curr.is_some(){Some(Box::new(curr.unwrap()))} else {None} 
            }
        })
    }

    pub fn pop(&mut self) {
        if let Some(elem) = self.head.take(){
            self.head = elem.next.map(|val| *val)
        }
    }

    pub fn len(&self) -> usize {
        match &self.head{
            Some(v) =>{
                let mut l = 0;
                let mut curr = &v.next;
                loop{
                    match curr{
                        Some(next) =>{
                            curr = &next.next;
                            l += 1;
                        },
                        None => {
                            break;
                        },
                    }
                }
                l+1
            },
            None => 0
        }
    }
}