use query_driver::MainDatabase;
use salsa::Snapshot;
use std::sync::Mutex;
use std::sync::MutexGuard;

#[derive(Debug)]
pub struct Context(Mutex<Snapshot<MainDatabase>>);

impl Context {
    pub fn new(db: Snapshot<MainDatabase>) -> Self {
        Context(Mutex::new(db))
    }

    pub fn get(&self) -> MutexGuard<Snapshot<MainDatabase>> {
        self.0.lock().unwrap()
    }
}

impl juniper::Context for Context {}
