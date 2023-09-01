use crate::async_trait;

pub mod redis;

#[async_trait]
pub trait Cache<K, V>: Clone + Send + Sync + 'static {
    async fn set(&self, key: K, value: &V) -> anyhow::Result<()>
    where
        K: 'async_trait,
        V: 'async_trait;

    async fn get(&self, key: K) -> anyhow::Result<Output<V>>
    where
        K: 'async_trait;
}

// Option<Option<_>> is gross and I just shot myself in the foot with it.
// TODO: Come up with a better name
pub enum Output<V> {
    Hit(V),
    Miss,
}

pub trait CachedExt: Sized {
    fn cached<C>(self, cache: C) -> Cached<Self, C> {
        Cached { inner: self, cache }
    }
}

impl<T> CachedExt for T {}

pub struct Cached<R, C> {
    pub inner: R,
    pub cache: C,
}
