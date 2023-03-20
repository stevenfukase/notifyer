#[derive(Debug)]
pub struct DateTime<T>(T);

impl<T> DateTime<T> {
    pub fn new(date_time: T) -> Self {
        Self(date_time)
    }
}
