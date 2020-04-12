use std::mem;

#[derive(Debug)]
pub struct BST<T> {
    root: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    elem: T,
}

type Link<T> = Option<Box<Node<T>>>;

trait InsertSearch<T: PartialOrd> {
    fn insert(&mut self, elem: T) -> bool;
    fn search(&self, elem: T) -> bool;
}

impl<T: PartialOrd> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, elem: T) -> bool {
        self.root.insert(elem)
    }

    pub fn search(&self, elem: T) -> bool {
        self.root.search(elem)
    }
}

impl<T: PartialOrd> InsertSearch<T> for Link<T> {
    fn insert(&mut self, elem: T) -> bool {
        match self {
            None => {
                let n = Some(Box::new(Node {
                    right: None,
                    left: None,
                    elem: elem,
                }));
                mem::replace(self, n);
                true
            }
            Some(n) => {
                if n.elem == elem {
                    false
                } else if elem < n.elem {
                    n.left.insert(elem)
                } else {
                    n.right.insert(elem)
                }
            }
        }
    }

    fn search(&self, elem: T) -> bool {
        match self {
            None => false,
            Some(n) => {
                if n.elem == elem {
                    true
                } else if elem < n.elem {
                    n.left.search(elem)
                } else {
                    n.right.search(elem)
                }
            }
        }
    }
}
// ==================== IntoIter

pub struct IntoIter<T> {
    cur_node: Link<T>,
}

impl<T: PartialOrd> IntoIterator for BST<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            cur_node: self.root,
        }
    }
}

impl<T: PartialOrd> Iterator for IntoIter<T> {
    type Item = T;

    // right-most traversal
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur_node.take() {
            None => None,
            Some(n) => {
                self.cur_node = n.right;
                Some(n.elem)
            }
        }
    }
}

// =================== Iter
pub struct Iter<'a, T: PartialOrd + 'a> {
    cur_node: Option<&'a Node<T>>,
}

impl<'a, T: PartialOrd + 'a> IntoIterator for &'a BST<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            cur_node: self.root.as_ref().map(|node| &**node),
        }
    }
}

impl<'a, T: PartialOrd> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // right-most traversal
    fn next(&mut self) -> Option<Self::Item> {
        self.cur_node.map(|node| {
            self.cur_node = node.right.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

// =================== Iter Mut
pub struct IterMut<'a, T: PartialOrd + 'a> {
    cur_node: Option<&'a mut Node<T>>,
}

impl<'a, T: PartialOrd + 'a> IntoIterator for &'a mut BST<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            cur_node: self.root.as_mut().map(|node| &mut **node),
        }
    }
}

impl<'a, T: PartialOrd> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    // right-most traversal
    fn next(&mut self) -> Option<Self::Item> {
        self.cur_node.take().map(|node| {
            self.cur_node = node.right.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
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

    #[test]
    fn into_iter() {
        let mut bst = BST::new();
        assert_eq!(bst.insert(4), true);
        assert_eq!(bst.insert(3), true);
        assert_eq!(bst.insert(5), true);
        assert_eq!(bst.insert(6), true);

        println!("{:?}", bst);
        let mut i = 0;
        for e in bst {
            assert_eq!(i + 4, e);
            i = i + 1;
        }
    }

    #[test]
    fn iter() {
        let mut bst = BST::new();
        assert_eq!(bst.insert(4), true);
        assert_eq!(bst.insert(3), true);
        assert_eq!(bst.insert(5), true);
        assert_eq!(bst.insert(6), true);

        for e in &bst {
            println!("{:?}", e);
        }
        let i = &mut bst.into_iter();
        assert_eq!(i.next(), Some(4));
        assert_eq!(i.next(), Some(5));
        assert_eq!(i.next(), Some(6));
        assert_eq!(i.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut bst = BST::new();
        assert_eq!(bst.insert(4), true);
        assert_eq!(bst.insert(3), true);
        assert_eq!(bst.insert(5), true);
        assert_eq!(bst.insert(6), true);

        for e in &mut bst {
            println!("{:?}", e);
        }
        let i = &mut bst.into_iter();
        assert_eq!(i.next(), Some(4));
        assert_eq!(i.next(), Some(5));
        assert_eq!(i.next(), Some(6));
        assert_eq!(i.next(), None);
    }
}
