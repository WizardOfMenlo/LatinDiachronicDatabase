pub trait GCollectable {
    fn garbage_sweep(&mut self);
    fn deep_sweep(&mut self);
}
