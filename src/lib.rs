mod stack;
mod syntree;

pub trait Stack {
    fn init() -> Self;

    fn push_val(&mut self, i: i32);

    fn top_val(&self) -> Option<&i32>;

    fn pop_val(&mut self) -> Option<i32>;

    fn is_empty(&self) -> bool;
}

pub use stack::ListStack;
pub use syntree::Syntree;
pub use syntree::ID;

#[cfg(test)]
mod tests {}
