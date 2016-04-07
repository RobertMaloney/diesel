use query_builder::{Query, CombinableQuery};

pub trait UnionDsl<
    U: CombinableQuery,
    Type = <U as Query>::SqlType
>: CombinableQuery {
    type Output: Query<SqlType=Type>;

    fn union(self, query: U) -> Self::Output;
    fn union_all(self, query: U) -> Self::Output;
}
