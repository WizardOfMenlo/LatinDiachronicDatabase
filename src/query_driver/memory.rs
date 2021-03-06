use crate::query_system::sources::GetLineQuery;

use super::MainDatabase;
use crate::query_system::gc::GCollectable;

use log::info;
use salsa::Database;

impl GCollectable for MainDatabase {
    fn garbage_sweep(&mut self) {
        info!("Sweeping garbage");

        /* TODO: Report salsa bug salsa-0.14.1\src\derived\slot.rs:510
        let sweep = SweepStrategy::default().discard_values();

        self.query(GetLineQuery).sweep(sweep);
        */
    }
}

pub(super) fn set_lru_sizes(db: &mut MainDatabase) {
    const MANY: usize = 256;

    db.query_mut(GetLineQuery).set_lru_capacity(MANY);
}
