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

impl<T> Default for SortStatement<T> {
    fn default() -> Self {
        SortStatement (vec![])
    }
}

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
    use crate::statement::IntoStatement;

    #[test]
    fn test_as_ref() {
        assert_eq!("asc", Sort::Asc(()).as_ref());
        assert_eq!("desc", Sort::Desc(()).as_ref());
    }

    #[test]
    fn test_statement() {
        #[derive(Serialize, Deserialize, AsRefStr)]
        #[serde(rename_all = "snake_case")]
        pub enum SortColumn {
            #[strum(serialize = "id")]
            Id,
            #[strum(serialize = "create_time")]
            CreateTime
        }

        let mut statement = SortStatement::<SortColumn>::default();
        statement.push(Sort::Desc(SortColumn::Id));
        statement.push(Sort::Asc(SortColumn::CreateTime));

        assert_eq!("id desc,create_time asc", statement.statement());
        assert_eq!("order by id desc,create_time asc", statement.full_statement());
    }
}