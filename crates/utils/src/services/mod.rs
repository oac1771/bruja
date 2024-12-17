pub mod contract_client;
pub mod job;
pub mod p2p;

#[cfg(test)]
pub mod test {
    type Func<T> = Box<dyn Fn() -> T + Send + Sync + 'static>;
    pub struct Expectation<T> {
        func: Option<Func<T>>,
    }

    impl<T> Expectation<T> {
        pub fn new() -> Self {
            Self { func: None }
        }

        pub fn func(self) -> Option<Func<T>> {
            self.func
        }

        pub fn _returns(&mut self, func: Func<T>) {
            self.func = Some(func);
        }
    }
}
