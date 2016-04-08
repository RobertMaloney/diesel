use query_builder::{Query, CombinableQuery, UnionStatement, IntersectStatement};
use query_dsl::*;

impl<L, R, Union, Type> UnionDsl<Union, Type>
    for IntersectStatement<L, R> where
    Union: CombinableQuery + Query<SqlType=Type>,
    IntersectStatement<L, R>: Query<SqlType=Type>,
{
    type Output = UnionStatement<IntersectStatement<L, R>, Union>;

    fn union(self, query: Union) -> Self::Output {
        UnionStatement::new(self, query, false)
    }

    fn union_all(self, query: Union) -> Self::Output {
        UnionStatement::new(self, query, true)
    }
}

impl<L, R, Intersect, Type> IntersectDsl<Intersect, Type>
    for IntersectStatement<L, R> where
    Intersect: CombinableQuery + Query<SqlType=Type>,
    IntersectStatement<L, R>: Query<SqlType=Type>,
{
    type Output = IntersectStatement<IntersectStatement<L, R>, Intersect>;

    fn intersect(self, query: Intersect) -> Self::Output {
        IntersectStatement::new(self, query, false)
    }

    fn intersect_all(self, query: Intersect) -> Self::Output {
        IntersectStatement::new(self, query, true)
    }
}
