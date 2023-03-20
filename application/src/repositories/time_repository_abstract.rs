pub trait TimeRepositoryAbstract {
    fn get_date(is_yesterday: bool) -> DateTime;
}
