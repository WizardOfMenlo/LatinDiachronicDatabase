use super::Context;
use crate::query_system::traits::*;
use juniper::FieldResult;
use systemstat::platform::common::Platform;
use systemstat::System;

pub struct Stats {}

#[derive(juniper::GraphQLObject)]
pub struct MemoryStats {
    total_memory_used: String,
    total_memory_free: String,
}

#[juniper::object(Context = Context)]
impl Stats {
    fn interned_words(context: &Context) -> FieldResult<i32> {
        Ok(context.get().word_db().len() as i32)
    }

    fn lemmatizer_forms(context: &Context) -> FieldResult<i32> {
        Ok(context.get().lemmatizer().num_forms() as i32)
    }

    fn lemmatizer_lemmas(context: &Context) -> FieldResult<i32> {
        Ok(context.get().lemmatizer().num_lemmas() as i32)
    }

    fn memory_usage(context: &Context) -> FieldResult<MemoryStats> {
        let sys = System::new();
        let mem = sys.memory().expect("Could not get sys info");
        Ok(MemoryStats {
            total_memory_used: mem.total.to_string_as(true),
            total_memory_free: mem.free.to_string_as(true),
        })
    }
}
