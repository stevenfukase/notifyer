use crate::repositories::contribution_count_repository_abstract::GetContributionCountRepositoryAbstract;
use crate::usecases::interfaces::AbstractUsecase;
use async_trait::async_trait;

pub struct GetContributionCountUsecase<'a> {
    repository: &'a dyn GetContributionCountRepositoryAbstract,
}

impl<'a> GetContributionCountUsecase<'a> {
    pub fn new(repository: &'a dyn GetContributionCountRepositoryAbstract) -> Self {
        GetContributionCountUsecase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<i64> for GetContributionCountUsecase<'a> {
    async fn execute(&self) -> Result<i64, std::io::Error> {
        self.repository.get_contribution_count().await
        // match contribution_count {
        //     Ok(contribution_count) => Ok(contribution_count),
        //     Err(e) => Err(e),
        // }
    }
}
