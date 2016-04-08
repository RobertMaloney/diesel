mod dsl_impls;

use backend::Backend;
use super::{Query, CombinableQuery, QueryBuilder, QueryFragment, BuildQueryResult};

pub struct ExceptStatement<L, R> {
    left: L,
    right: R,
    all: bool,
}

impl<L, R> ExceptStatement<L, R> {
    pub fn new(left: L, right: R, all: bool) -> Self {
        ExceptStatement {
            left: left,
            right: right,
            all: all,
        }
    }
}

impl<L, R> Query for ExceptStatement<L, R> where
    L: CombinableQuery,
    R: CombinableQuery,
{
    type SqlType = <L as Query>::SqlType;
}

impl<L, R> CombinableQuery for ExceptStatement<L, R> where
    ExceptStatement<L, R>: Query,
{
}

impl<L, R, DB> QueryFragment<DB> for ExceptStatement<L, R> where
    DB: Backend,
    L: QueryFragment<DB>,
    R: QueryFragment<DB>,
{
    fn to_sql(&self, out: &mut DB::QueryBuilder) -> BuildQueryResult {
        out.push_sql("(");
        try!(self.left.to_sql(out));
        if self.all {
            out.push_sql(" EXCEPT ALL ");
        } else {
            out.push_sql(" EXCEPT ");
        }
        try!(self.right.to_sql(out));
        out.push_sql(")");
        Ok(())
    }
}
