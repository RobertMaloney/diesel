extern crate libsqlite3_sys as ffi;
extern crate libc;

#[doc(hidden)]
pub mod raw;
mod stmt;
mod statement_iterator;

use std::ffi::CStr;

use backend::Sqlite;
use query_builder::*;
use query_builder::sqlite::SqliteQueryBuilder;
use query_source::*;
use result::*;
use result::Error::QueryBuilderError;
use self::raw::*;
use self::stmt::*;
use super::{SimpleConnection, Connection};
use types::HasSqlType;

pub struct SqliteConnection {
    raw_connection: RawConnection,
}

impl SimpleConnection for SqliteConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.raw_connection.exec(query)
    }
}

impl Connection for SqliteConnection {
    type Backend = Sqlite;

    fn establish(database_url: &str) -> ConnectionResult<Self> {
        RawConnection::establish(database_url).map(|conn| {
            SqliteConnection {
                raw_connection: conn,
            }
        })
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        try!(self.batch_execute(query));
        Ok(self.raw_connection.rows_affected_by_last_query())
    }

    fn query_all<'a, T, U: 'a>(&self, _source: T) -> QueryResult<Box<Iterator<Item=U> + 'a>> where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend>,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend>,
    {
        let sql = try!(self.prepare_query(&source.as_query()));
        let stmt = try!(Statement::prepare(&self.raw_connection, &sql));
        // Box::new(StatementIterator::new(stmt)) as Box<Iterator<Item=U>>
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize> where
        T: QueryFragment<Self::Backend>,
    {
        let stmt = try!(self.prepare_query(source));
        try!(stmt.run());
        Ok(self.raw_connection.rows_affected_by_last_query())
    }

    fn silence_notices<F: FnOnce() -> T, T>(&self, _f: F) -> T {
        unimplemented!()
    }

    fn begin_transaction(&self) -> QueryResult<()>{
        unimplemented!()
    }

    fn rollback_transaction(&self) -> QueryResult<()> {
        unimplemented!()
    }

    fn commit_transaction(&self) -> QueryResult<()> {
        unimplemented!()
    }

    fn get_transaction_depth(&self) -> i32 {
        unimplemented!()
    }
}

impl SqliteConnection {
    fn prepare_query<T: QueryFragment<Sqlite>>(&self, source: &T) -> QueryResult<Statement> {
        let mut query_builder = SqliteQueryBuilder::new();
        try!(source.to_sql(&mut query_builder).map_err(QueryBuilderError));
        let mut result = try!(Statement::prepare(&self.raw_connection, &query_builder.sql));

        for (tpe, value) in query_builder.bind_params.into_iter() {
            try!(result.bind(tpe, value));
        }

        Ok(result)
    }
}

fn error_message(err_code: libc::c_int) -> &'static str {
    let message_ptr = unsafe { ffi::sqlite3_errstr(err_code) };
    let result = unsafe { CStr::from_ptr(message_ptr) };
    result.to_str().unwrap()
}
