use crate::Stack;

// TODO Complete implementation
impl Stack for Vec<i32> {
    fn init() -> Self {
        todo!()
    }

    fn push_val(&mut self, i: i32) {
        todo!()
    }

    fn top_val(&self) -> Option<&i32> {
        todo!()
    }

    fn pop_val(&mut self) -> Option<i32> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}

#[derive(Debug)]
pub enum ListStack {
    Val(i32, Option<Box<ListStack>>),
    Nil,
}

use ListStack::Nil;
use ListStack::Val;

// Complete implementation of Stack for ListStack
impl Stack for ListStack {
    fn init() -> Self {
        Nil
    }

    fn push_val(&mut self, i: i32) {
        match self {
            Val(value, other) => *self = todo!(),
            Nil => *self = todo!(),
        };
    }

    fn top_val(&self) -> Option<&i32> {
        todo!()
    }

    fn pop_val(&mut self) -> Option<i32> {
        match self {
            Val(value, other) => {
                let popped_value = *value;
                match other.take() {
                    None => *self = Nil,
                    Some(other) => todo!(),
                };
                todo!()
            }
            Nil => None,
        }
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::ListStack;
    use crate::Stack;
    use std::fmt::Debug;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    #[test]
    fn fill_and_clear() {
        println! {"Testing ListStack"}
        fill_and_clear_impl(ListStack::init());
        println! {"Testing Vec<T>"}
        fill_and_clear_impl(Vec::init());
    }

    fn fill_and_clear_impl<T: Stack + Debug>(mut stack: T) {
        stack.push_val(1);
        assert_eq!(stack.top_val(), Some(&1));

        stack.push_val(2);
        assert_eq!(stack.top_val(), Some(&2));

        stack.push_val(-3);
        assert_eq!(stack.top_val(), Some(&-3));

        println!("{:?}", stack);

        let mut comparison = vec![1, 2, -3];
        while let Some(val) = stack.pop_val() {
            assert_eq!(comparison.pop().unwrap(), val);
        }

        assert!(stack.is_empty())
    }

    const BENCHMARK_SIZE: i32 = 10_000_000;
    #[test]
    fn benchmark() {
        let (pushed, popped) = bench(ListStack::init());
        println!(
            "Own implementation took {}s for push and {} for pop.",
            pushed, popped
        );

        println!("Sleeping for 10 seconds.");
        sleep(Duration::from_secs(10));

        let (pushed, popped) = bench(Vec::init());
        println!(
            "Vec wrapper took {}s for push and {} for pop.",
            pushed, popped
        );
    }

    fn bench<T: Stack>(mut stack: T) -> (u64, u64) {
        let start = Instant::now();
        for i in 1..BENCHMARK_SIZE {
            stack.push_val(i);
        }
        println!("Pushed all elements");
        let pushed = start.elapsed().as_secs();
        while stack.pop_val().is_some() {}
        println!("Popped all elements");
        let popped = start.elapsed().as_secs();
        (pushed, popped)
    }

    #[test]
    fn test_mem() {
        let stack = ListStack::init();
        mem_test(stack);
        println!("Finished memory test for ListStack");

        println!("Sleeping for 10 seconds.");
        sleep(Duration::from_secs(10));

        let stack = Vec::init();
        mem_test(stack);
        println!("Finished memory test for Vec<T>");
    }

    fn mem_test<T: Stack>(mut stack: T) {
        for i in 1..BENCHMARK_SIZE {
            stack.push_val(i);
            stack.pop_val();
        }
        println!("Completed memory test elements");
    }
}
