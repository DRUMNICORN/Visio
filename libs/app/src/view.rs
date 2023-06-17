use async_trait::async_trait;

#[async_trait]
pub trait NodiumView: Send + Sync {
    async fn run(
        &self,
        // event_callback: Box<dyn Fn(NodiumEvent) + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
