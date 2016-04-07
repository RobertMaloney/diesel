mod dsl_impls;

use backend::Backend;
use super::{Query, CombinableQuery, QueryBuilder, QueryFragment, BuildQueryResult};

pub struct UnionStatement<L, R> {
    left: L,
    right: R,
    all: bool,
}

impl<L, R> UnionStatement<L, R> {
    pub fn new(left: L, right: R, all: bool) -> Self {
        UnionStatement {
            left: left,
            right: right,
            all: all,
        }
    }
}

impl<L, R> Query for UnionStatement<L, R> where
    L: CombinableQuery,
    R: CombinableQuery,
{
    type SqlType = <L as Query>::SqlType;
}

impl<L, R> CombinableQuery for UnionStatement<L, R> where
    UnionStatement<L, R>: Query,
{
}

impl<L, R, DB> QueryFragment<DB> for UnionStatement<L, R> where
    DB: Backend,
    L: QueryFragment<DB>,
    R: QueryFragment<DB>,
{
    fn to_sql(&self, out: &mut DB::QueryBuilder) -> BuildQueryResult {
        out.push_sql("(");
        try!(self.left.to_sql(out));
        if self.all {
            out.push_sql(" UNION ALL ");
        } else {
            out.push_sql(" UNION ");
        }
        try!(self.right.to_sql(out));
        out.push_sql(")");
        Ok(())
    }
}
