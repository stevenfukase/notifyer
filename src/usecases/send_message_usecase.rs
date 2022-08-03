pub struct SendMessageUsecase<'a> {
    repository: &'a dyn MessageRepositoryAbstract,
}

impl<'a> SendMessageUsecase<'a> {
    pub fn new(repository: &'a dyn MessageRepositoryAbstract) -> Self {
        SendMessageUsecase { repository }
    }
}

impl<'a> AbstractUsecase for SendMessageUsecase<'a> {

}