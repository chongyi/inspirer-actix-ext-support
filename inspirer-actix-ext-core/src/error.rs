use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Cannot found dependency: {0}")]
    DependencyNotFound(&'static str),
}