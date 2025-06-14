// use diesel::backend::Backend;
// use diesel::serialize::{IsNull};
// use std::io::Write;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::AsExpression;
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;

#[repr(i32)]
#[derive(Debug, Clone, Copy, AsExpression, PartialEq, Eq)]
#[diesel(sql_type = Integer)]
pub enum TaskStatus {
    Todo = 0,
    InProgress = 1,
    Done = 2,
}


impl ToSql<Integer, diesel::sqlite::Sqlite> for TaskStatus {
    fn to_sql(&self, out: &mut Output<'_, '_, diesel::sqlite::Sqlite>) -> serialize::Result {
        let value = match *self {
            // you can call a trait method for a specific type using <Type as Trait>::method
            // ToSql<Integer, Sqlite> is a Trait provided by Diesel
            // e.g. ToSql<Integer, Sqlite>>::to_sql(&0, out)
            // You can implement from an instance of the data type
            // e.g. 0.to_sql(out)
            // or you can implement explicitly using a type, here the type is i32
            // e.g. <i32 as ToSql<Integer, Sqlite>>::to_sql(&0, out)
            // Explanation of <Type as Trait> Rust syntax. 
            //    Use the implementation of the ToSql trait for the type i32
            //    with the SQL type Integer and the backend Sqlite.
            TaskStatus::Todo => <i32 as ToSql<Integer, Sqlite>>::to_sql(&0, out),
            TaskStatus::InProgress => <i32 as ToSql<Integer, Sqlite>>::to_sql(&1, out),
            TaskStatus::Done => <i32 as ToSql<Integer, Sqlite>>::to_sql(&2, out)
        };
        
        value
    }
}

impl FromSql<Integer, diesel::sqlite::Sqlite> for TaskStatus {
    fn from_sql(bytes: <diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
            0 => Ok(TaskStatus::Todo),
            1 => Ok(TaskStatus::InProgress),
            2 => Ok(TaskStatus::Done),
            x => Err(format!("Unknown task status: {}", x).into())
        }
    }
}