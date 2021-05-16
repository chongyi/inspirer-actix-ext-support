#[macro_use]
extern crate log;
#[macro_use]
extern crate async_trait;

pub mod config;
mod module_provider;

pub use module_provider::mysql;

use sqlx::{Executor, Database};

/// 数据访问对象查询器
///
/// 实现该 Trait 的对象都可以作为查询器被调度
#[async_trait]
pub trait DAO<D: Database> {
    type Result;

    async fn run<'a, E>(&self, executor: E) -> Self::Result
        where E: Executor<'a, Database=D>
    ;
}

pub mod prelude {
    pub use sqlx;
    pub use crate::DAO;
    pub use crate::module_provider::mysql;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
