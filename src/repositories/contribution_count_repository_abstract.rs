use async_trait::async_trait;

#[async_trait(?Send)]
pub trait GetContributionCountRepositoryAbstract {
    async fn get_contribution_count(&self) -> Result<i64, std::io::Error>;
}
