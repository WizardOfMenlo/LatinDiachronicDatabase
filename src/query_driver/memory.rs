use crate::query_system::{
            middle::{
                FormOccurrencesSubsetQuery, FormsInSourceQuery, FormsInSubsetQuery,
                LemmaOccurrencesSubsetQuery, LemmasInSourceQuery, LemmasInSubsetQuery,
                ParseSubsetQuery, SourceTreeQuery, SubsetTreeQuery,
            },
            sources::GetLineQuery,
        };

use super::MainDatabase;
use crate::query_system::gc::GCollectable;

use log::info;
use salsa::{SweepStrategy, Database, Durability};


impl GCollectable for MainDatabase {
    fn garbage_sweep(&mut self) {
        info!("Sweeping garbage");

        self.salsa_runtime_mut().synthetic_write(Durability::MEDIUM);

        let sweep = SweepStrategy::discard_outdated();

        self.query(GetLineQuery).sweep(sweep);
        self.query(ParseSubsetQuery).sweep(sweep);
        self.query(FormsInSubsetQuery).sweep(sweep);
        self.query(FormsInSourceQuery).sweep(sweep);
        self.query(LemmasInSubsetQuery).sweep(sweep);
        self.query(LemmasInSourceQuery).sweep(sweep);
        self.query(SourceTreeQuery).sweep(sweep);
        self.query(SubsetTreeQuery).sweep(sweep);
        self.query(FormOccurrencesSubsetQuery).sweep(sweep);
        self.query(LemmaOccurrencesSubsetQuery).sweep(sweep);
    }

    fn deep_sweep(&mut self) {
        info!("Deep sweep");
        self.salsa_runtime_mut().synthetic_write(Durability::HIGH);
        self.sweep_all(SweepStrategy::discard_outdated());
    }
}

pub(super) fn set_lru_sizes(db: &mut MainDatabase) {
    const FEW: usize = 32;
    const MANY: usize = 256;

    db.query_mut(GetLineQuery).set_lru_capacity(MANY);
    db.query_mut(ParseSubsetQuery).set_lru_capacity(FEW);
    db.query_mut(FormsInSubsetQuery).set_lru_capacity(FEW);
    db.query_mut(FormsInSourceQuery).set_lru_capacity(FEW);
    db.query_mut(LemmasInSubsetQuery).set_lru_capacity(FEW);
    db.query_mut(LemmasInSourceQuery).set_lru_capacity(FEW);
    db.query_mut(SourceTreeQuery).set_lru_capacity(FEW);
    db.query_mut(SubsetTreeQuery).set_lru_capacity(FEW);
    db.query_mut(FormOccurrencesSubsetQuery).set_lru_capacity(FEW);
    db.query_mut(LemmaOccurrencesSubsetQuery).set_lru_capacity(FEW);
}