use sqlx::Database;

pub mod condition;
pub mod sort;

pub trait IntoStatement<T: Database> {
    fn statement(&self) -> String;
    fn full_statement(&self) -> String;
}