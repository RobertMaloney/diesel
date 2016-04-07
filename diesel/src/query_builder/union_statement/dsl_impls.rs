use query_builder::{Query, CombinableQuery, UnionStatement};
use query_dsl::*;

impl<L, R, Union, Type> UnionDsl<Union, Type>
    for UnionStatement<L, R> where
    Union: CombinableQuery + Query<SqlType=Type>,
    UnionStatement<L, R>: Query<SqlType=Type>,
{
    type Output = UnionStatement<UnionStatement<L, R>, Union>;

    fn union(self, query: Union) -> Self::Output {
        UnionStatement::new(self, query, false)
    }

    fn union_all(self, query: Union) -> Self::Output {
        UnionStatement::new(self, query, true)
    }
}
