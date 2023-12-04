use {
    super::{Cache, Output},
    crate::util::redis,
    anyhow::Context as _,
    async_trait::async_trait,
    serde::{Deserialize, Serialize},
};

const TTL_SECS: usize = 300;

#[async_trait]
impl<K, V> Cache<K, V> for redis::Adapter
where
    K: AsRef<str> + Send + Sync,
    for<'de> V: Serialize + Deserialize<'de> + Send + Sync,
{
    async fn set(&self, key: &K, value: &V) -> anyhow::Result<()>
    where
        K: 'async_trait,
        V: 'async_trait,
    {
        let bytes = rmp_serde::to_vec(value).context("Failed to serialize data")?;
        self.set_ex(key.as_ref(), bytes, TTL_SECS).await
    }

    async fn get(&self, key: &K) -> anyhow::Result<Output<V>>
    where
        K: 'async_trait,
    {
        let output: Option<Vec<u8>> = self.get(key.as_ref()).await?;
        Ok(match output {
            Some(bytes) => rmp_serde::from_slice::<V>(bytes.as_slice())
                .map(Output::Hit)
                .context("Failed to deserialize data")?,
            None => Output::Miss,
        })
    }
}
