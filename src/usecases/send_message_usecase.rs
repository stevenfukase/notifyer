pub struct SendMessageUsecase<'a> {
    repository: &'a dyn MessageRepositoryAbstract,
}

impl<'a> SendMessageUsecase<'a> {
    pub fn new(repository: &'a dyn MessageRepositoryAbstract) -> Self {
        SendMessageUsecase { repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUsecase<Ok> for SendMessageUsecase<'a> {
    async fn execute(&self) -> Result<Ok, Error> {
        let result = self.repository.send().await;
        match result {
            Ok(_) => Ok(_),
            Err(_) => Err(_),
        }
    }
}