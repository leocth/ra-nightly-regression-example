#[async_trait::async_trait]
pub trait Example {
    async fn test(&self) -> u32;
}
