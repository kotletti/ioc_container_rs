use std::{any::TypeId, cell::RefCell};

thread_local! {
    static BUILDING: RefCell<Vec<TypeId>> = const { RefCell::new(Vec::new()) };
}

pub struct ChainGuard;

impl ChainGuard {
    pub fn enter(key: TypeId) -> Option<ChainGuard> {
        BUILDING.with(|stack| {
            let mut stack = stack.borrow_mut();

            if stack.contains(&key) {
                return None;
            }

            stack.push(key);

            Some(ChainGuard)
        })
    }
}

impl Drop for ChainGuard {
    fn drop(&mut self) {
        BUILDING.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}
