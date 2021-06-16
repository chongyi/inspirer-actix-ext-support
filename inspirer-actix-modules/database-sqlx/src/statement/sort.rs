use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize, AsRefStr)]
#[serde(tag = "mode", content = "column")]
pub enum Sort<T> {
    #[serde(rename = "asc")]
    #[strum(serialize = "asc")]
    Asc(T),
    #[serde(rename = "desc")]
    #[strum(serialize = "desc")]
    Desc(T),
}

pub struct SortStatement<T> (Vec<Sort<T>>);

impl<T> Deref for SortStatement<T> {
    type Target = Vec<Sort<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for SortStatement<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub mod mysql {
    use super::{Sort, SortStatement};
    use crate::statement::IntoStatement;
    use sqlx::MySql;

    impl<T: AsRef<str>> IntoStatement<MySql> for SortStatement<T> {
        fn statement(&self) -> String {
            self.0.iter()
                .map(|option| match option {
                    Sort::Asc(field) => format!("{} asc", field.as_ref()),
                    Sort::Desc(field) => format!("{} desc", field.as_ref()),
                })
                .collect::<Vec<String>>()
                .join(",")
        }

        fn full_statement(&self) -> String {
            let statement = self.statement();
            if statement.is_empty() {
                statement
            } else {
                format!("order by {}", statement)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_ref() {
        assert_eq!("asc", Sort::Asc(()));
        assert_eq!("desc", Sort::Desc(()));
    }
}