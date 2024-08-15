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
    _data: Option<Vec<u8>>,
    _created_at: Option<PrimitiveDateTime>,
}

impl FromRow for FindPeopleResult {
    fn from_row_opt(row: Row) -> Result<Self, FromRowError> {
        Err(FromRowError(row.clone()))
        /*
        Ok(FindPeopleResult {
            _id: row.get(0).unwrap(),
            _name: row.get(1).unwrap(),
            _age: row.get(2).unwrap(),
            _hash: row.get(3).unwrap(),
            _available: row.get(4).unwrap(),
            _data: row.get(5),
            _created_at: row.get(6),
        })
         */
    }
}

impl SelectQuery<FindPeopleResult> for FindPeople {
    fn sql(&self) -> String {
        r#"SELECT
            id
        ,   name
        ,   age
        ,   hash
        ,   available
        ,   data
        ,   created_at
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
