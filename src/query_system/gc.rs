pub trait GCollectable {
    fn garbage_sweep(&mut self);
}
