#[derive(Serialize, Deserialize, AsRefStr, Debug, Clone)]
#[serde(tag = "mode", content = "column")]
pub enum Sort<T> {
    #[serde(rename = "asc")]
    #[strum(serialize = "asc")]
    Asc(T),
    #[serde(rename = "desc")]
    #[strum(serialize = "desc")]
    Desc(T),
}

pub type SortStatement<T> = Vec<Sort<T>>;

pub mod mysql {
    use super::{Sort, SortStatement};
    use crate::statement::IntoStatement;
    use sqlx::MySql;

    impl<T: AsRef<str>> IntoStatement<MySql> for SortStatement<T> {
        fn statement(&self) -> String {
            self.iter()
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

    #[derive(Serialize, Deserialize, AsRefStr, PartialEq, Debug)]
    #[serde(rename_all = "snake_case")]
    pub enum SortColumn {
        #[strum(serialize = "content.id")]
        Id,
        #[strum(serialize = "content.create_time")]
        CreateTime,
    }

    #[test]
    fn test_as_ref() {
        assert_eq!("asc", Sort::Asc(()).as_ref());
        assert_eq!("desc", Sort::Desc(()).as_ref());
    }

    #[test]
    fn test_statement() {
        let mut statement = SortStatement::<SortColumn>::default();
        statement.push(Sort::Desc(SortColumn::Id));
        statement.push(Sort::Asc(SortColumn::CreateTime));

        assert_eq!(
            "content.id desc,content.create_time asc",
            statement.statement()
        );
        assert_eq!(
            "order by content.id desc,content.create_time asc",
            statement.full_statement()
        );
    }

    #[test]
    fn test_serialize() {
        let mut statement = SortStatement::<SortColumn>::default();
        statement.push(Sort::Desc(SortColumn::Id));
        statement.push(Sort::Asc(SortColumn::CreateTime));

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Options {
            sorts: SortStatement<SortColumn>,
        }

        assert_eq!(
            "sorts[0][mode]=desc&sorts[0][column]=id&sorts[1][mode]=asc&sorts[1][column]=create_time", 
            serde_qs::to_string(&Options { sorts: statement}).unwrap()
        );

        let a:Sort<SortColumn> = serde_qs::from_str("mode=desc&column=id").unwrap();
        if let Sort::Desc(inner) = a {
            assert_eq!("content.id", inner.as_ref());
        } else {
            assert!(false);
        }

        let b = serde_qs::from_str::<Options>("sorts[0][mode]=desc&sorts[0][column]=id&sorts[1][mode]=asc&sorts[1][column]=create_time");
        assert!(b.is_ok());
        println!("{:?}", b);
    }
}
