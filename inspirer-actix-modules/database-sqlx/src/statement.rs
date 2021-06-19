use sqlx::Database;

pub mod condition;
pub mod sort;
pub mod pagination;

pub trait IntoStatement<T: Database> {
    fn statement(&self) -> String;
    fn full_statement(&self) -> String;
}

pub trait UpdateArgument {

}