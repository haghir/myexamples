use mysql_async::{prelude::*, params::Params, Row, FromRowError};
use time::PrimitiveDateTime;

use crate::wrapper::SelectQuery;

pub(crate) struct FindPeople {
    pub name: String,
}

#[derive(Debug)]
pub(crate) struct FindPeopleResult {
    _id: u64,
    _name: String,
    _age: Option<i32>,
    _hash: Option<String>,
    _available: bool,
    _created_at: Option<PrimitiveDateTime>,
}

impl FromRow for FindPeopleResult {
    fn from_row_opt(mut row: Row) -> core::result::Result<Self, FromRowError> {
        Ok(FindPeopleResult {
            _id: row.take(0).unwrap(),
            _name: row.take(1).unwrap(),
            _age: row.take(2).unwrap(),
            _hash: row.take(3).unwrap(),
            _available: row.take(4).unwrap(),
            _created_at: row.take(5).unwrap(),
        })
    }
}

impl SelectQuery<FindPeopleResult> for FindPeople {
    fn sql(&self) -> String {
        r#"SELECT
            id
        ,   name
        ,   age
        ,   hash
        , available
        , created_at
        FROM
            people
        WHERE
            name LIKE :name
        ;"#.to_string()
    }

    fn params(&self) -> Params {
        params! {
            "name" => self.name.clone(),
        }
    }
}
