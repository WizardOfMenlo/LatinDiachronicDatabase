use query_driver::MainDatabase;
use salsa::ParallelDatabase;
use salsa::Snapshot;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

#[derive(Clone)]
pub struct Context(Arc<Mutex<Snapshot<MainDatabase>>>);

impl Context {
    pub fn new(db: &MainDatabase) -> Self {
        Context(Arc::new(Mutex::new(db.snapshot())))
    }

    pub fn get(&self) -> MutexGuard<Snapshot<MainDatabase>> {
        self.0.lock().unwrap()
    }
}

impl juniper::Context for Context {}
