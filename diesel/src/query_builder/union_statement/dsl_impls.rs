use query_builder::{Query, CombinableQuery, UnionStatement, IntersectStatement, ExceptStatement};
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

impl<L, R, Intersect, Type> IntersectDsl<Intersect, Type>
    for UnionStatement<L, R> where
    Intersect: CombinableQuery + Query<SqlType=Type>,
    UnionStatement<L, R>: Query<SqlType=Type>,
{
    type Output = IntersectStatement<UnionStatement<L, R>, Intersect>;

    fn intersect(self, query: Intersect) -> Self::Output {
        IntersectStatement::new(self, query, false)
    }

    fn intersect_all(self, query: Intersect) -> Self::Output {
        IntersectStatement::new(self, query, true)
    }
}

impl<L, R, Except, Type> ExceptDsl<Except, Type>
    for UnionStatement<L, R> where
    Except: CombinableQuery + Query<SqlType=Type>,
    UnionStatement<L, R>: Query<SqlType=Type>,
{
    type Output = ExceptStatement<UnionStatement<L, R>, Except>;

    fn except(self, query: Except) -> Self::Output {
        ExceptStatement::new(self, query, false)
    }

    fn except_all(self, query: Except) -> Self::Output {
        ExceptStatement::new(self, query, true)
    }
}
