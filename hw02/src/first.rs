use std::mem;

#[derive(Debug)]
pub struct BST {
    root: Link,
}

#[derive(Debug)]
struct Node {
    left: Link,
    right: Link,
    elem: i32,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

impl BST {
    pub fn new() -> Self {
        BST { root: Link::Empty }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: i32) -> bool {
        self.root.search(elem)
    }
}

impl Link {
    fn insert(&mut self, elem: i32) -> bool {
        match self {
            Link::Empty => {
                let n = Link::More(Box::new(Node {
                    right: Link::Empty,
                    left: Link::Empty,
                    elem: elem,
                }));
                mem::replace(self, n);
                true
            }
            Link::More(n) => {
                if n.elem == elem {
                    false
                } else if n.elem < elem {
                    n.left.insert(elem)
                } else {
                    n.right.insert(elem)
                }
            }
        }
    }

    fn search(&self, elem: i32) -> bool {
        match self {
            Link::Empty => false,
            Link::More(n) => {
                if n.elem == elem {
                    true
                } else if n.elem < elem {
                    n.left.search(elem)
                } else {
                    n.right.search(elem)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut bst = BST::new();
        assert_eq!(bst.insert(1), true);
        assert_eq!(bst.insert(2), true);
        assert_eq!(bst.insert(2), false);

        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(5), false);
    }
}
