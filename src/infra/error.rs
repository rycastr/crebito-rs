#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    UnprocessableEntity(String),
}
