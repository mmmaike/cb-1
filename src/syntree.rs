use std::{
    borrow::BorrowMut,
    fmt::{Display, Formatter},
    ops::Deref,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ID(usize);

#[derive(Debug, PartialEq)]
pub struct Syntree<T> {
    id: ID,
    value: T,
    children: Vec<Syntree<T>>,
}

// Complete the implementation
// Hint: Start with seek_node_mut
impl<'a, T> Syntree<T> {
    pub fn new(value: T, id: ID) -> Syntree<T> {
        todo!()
    }

    pub fn push_node(&mut self, parent_id: ID, new_node: Syntree<T>) -> Result<(), String> {
        todo!()
    }

    pub fn prepend_node(&mut self, parent_id: ID, new_node: Syntree<T>) -> Result<(), String> {
        todo!()
    }

    pub fn insert_node(
        &mut self,
        parent_id: ID,
        index: usize,
        new_node: Syntree<T>,
    ) -> Result<(), String> {
        todo!()
    }

    // Anmerkung: `'a` Is ein Lebenszeit angabe für die Referenzen
    // Hier wird einfach nur explizit gesagt: Solange `self` lebt, lebt auch die Referenz im Rückgabewert
    pub fn seek_node(&'a self, id: &ID) -> Option<&'a Syntree<T>> {
        if self.id == *id {
            Some(self)
        } else {
            for child in &self.children {
                if let Some(result) = child.seek_node(id) {
                    return Some(result);
                }
            }
            None
        }
    }

    pub fn seek_node_mut(&'a mut self, id: &ID) -> Option<&'a mut Syntree<T>> {
        //let found_node = self.seek_node(id).clone();
        // let found_node = self.seek_node(id).take();
        // let mut mut_node = found_node.unwrap().deref().clone();
        // let unwrapped_syntree = &mut mut_node;
        // let unwrapped = unwrapped_syntree;
        // Some(&mut self.seek_node(id).take().unwrap())

        if self.id == *id {
            Some(self)
        } else {
            for child in &mut self.children {
                if let Some(result) = &mut child.seek_node(id) {
                    return Some(result);
                }
            }
            None
        }
    }
}

impl<T: Display> Syntree<T> {
    pub fn print(&self) -> String {
        if self.children.is_empty() {
            format!("{}", self.value)
        } else {
            format!(
                "{}-[{}]",
                self.value,
                &self
                    .children
                    .iter()
                    .map(|tn| tn.print())
                    .collect::<Vec<String>>()
                    .join(",")
            )
        }
    }
}

impl<T: Display> Display for Syntree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.print())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_tree() -> Result<(), String> {
        let mut tree = Syntree::new(0, ID(0));

        for child in 1..3 {
            let child_id = ID(child);
            let mut child = Syntree::new(child, child_id);
            for grandchild in 1..3 {
                let id = grandchild * 10;
                child.prepend_node(child_id, Syntree::new(id, ID(id)))?;
            }
            tree.push_node(ID(0), child)?;
        }
        println!("{}", tree);
        assert_eq!(String::from("0-[1-[20,10],2-[20,10]]"), tree.print());
        Ok(())
    }

    #[test]
    fn fill_tree_words() -> Result<(), String> {
        let mut tree = Syntree::new(to_s("root"), ID(0));

        for (child_id, child) in ["first", "second", "third"].iter().map(to_s).enumerate() {
            let child_id = ID(child_id);
            let mut child = Syntree::new(child, child_id);
            if child_id.0 == 0 {
                let descendant = Syntree::new(to_s("A"), ID(4));
                child.push_node(child_id, descendant)?;
                let descendant = Syntree::new(to_s("B"), ID(5));
                child.push_node(ID(4), descendant)?;
                let descendant = Syntree::new(to_s("C"), ID(6));
                child.push_node(ID(5), descendant)?;
            }
            tree.push_node(ID(0), child)?;
        }
        println!("{}", tree);
        assert_eq!(
            String::from("root-[first-[A-[B-[C]]],second,third]"),
            tree.print()
        );
        Ok(())
    }

    fn to_s<T: Display>(value: T) -> String {
        format!("{}", value)
    }

    #[test]
    fn node_id_not_found() -> Result<(), String> {
        let mut tree = Syntree::new(0, ID(0));
        let child_id = ID(1);
        let child = Syntree::new(1, child_id);
        tree.push_node(ID(0), child)?;

        let child_id = ID(2);
        let child = Syntree::new(3, child_id);
        assert!(tree.push_node(ID(5), child).is_err());
        Ok(())
    }
}
