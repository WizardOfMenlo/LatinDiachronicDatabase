use super::query_system::ids::SourceId;

use bimap::BiMap;
//use notify::{EventFn, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;

pub trait FileSystem {
    type Source;

    fn load(&self, id: SourceId) -> String;
    fn watch(&self, id: SourceId);

    fn intern_source(&mut self, path: Self::Source) -> SourceId;
}

// TODO: This might just be AsRef
pub trait GetFileSystem {
    type Fs: FileSystem;

    fn filesystem(&self) -> &Self::Fs;
    fn filesystem_mut(&mut self) -> &mut Self::Fs;
}

impl<T> FileSystem for T
where
    T: GetFileSystem,
{
    type Source = <T::Fs as FileSystem>::Source;

    fn intern_source(&mut self, path: Self::Source) -> SourceId {
        self.filesystem_mut().intern_source(path)
    }

    fn watch(&self, id: SourceId) {
        self.filesystem().watch(id)
    }

    fn load(&self, id: SourceId) -> String {
        self.filesystem().load(id)
    }
}

#[derive(Debug, Default, Clone)]
pub struct InternerFileSystem {
    sources: BiMap<PathBuf, SourceId>,
}

impl InternerFileSystem {
    pub fn new() -> Self {
        InternerFileSystem {
            sources: BiMap::new(),
        }
    }

    pub fn sources(&self) -> &BiMap<PathBuf, SourceId> {
        &self.sources
    }
}

impl FileSystem for InternerFileSystem {
    type Source = PathBuf;

    fn intern_source(&mut self, path: Self::Source) -> SourceId {
        if self.sources.contains_left(&path) {
            return *self.sources.get_by_left(&path).unwrap();
        }

        // Guarantee that we never overwrite a value
        let mut new_id = self.sources.len() as u32;
        while self.sources.contains_right(&SourceId::from_integer(new_id)) {
            new_id += 1;
        }

        let new_id = SourceId::from_integer(new_id);
        self.sources.insert(path, new_id);
        new_id
    }

    fn watch(&self, id: SourceId) {
        let _path = self.sources.get_by_right(&id).unwrap();
        /*
        // TODO: Error recovery
        self.watcher
            .watch(path, RecursiveMode::NonRecursive)
            .unwrap();
        */
    }

    fn load(&self, id: SourceId) -> String {
        // TODO: Check conversions
        std::fs::read_to_string(self.sources.get_by_right(&id).unwrap()).unwrap()
    }
}

#[derive(Debug, Default, Clone)]
pub struct MockFileSystem {
    strings: BiMap<String, SourceId>,
}

impl FileSystem for MockFileSystem {
    type Source = String;

    fn intern_source(&mut self, path: Self::Source) -> SourceId {
        // TODO: Bound check
        let new_id = SourceId::from_integer(self.strings.len() as u32);
        self.strings.insert(path, new_id);
        new_id
    }

    fn watch(&self, _: SourceId) {}

    fn load(&self, id: SourceId) -> String {
        self.strings.get_by_right(&id).unwrap().clone()
    }
}
