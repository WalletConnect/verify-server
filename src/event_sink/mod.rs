pub mod s3;

pub trait EventSink<Ev>: Send + Sync + 'static {
    fn send(&self, ev: Ev);
}
