use query_driver::MainDatabase;

#[derive(shrinkwraprs::Shrinkwrap)]
pub struct Context(MainDatabase);

impl Context {
    pub fn new(db: MainDatabase) -> Self {
        Context(db)
    }
}

impl juniper::Context for Context {}
