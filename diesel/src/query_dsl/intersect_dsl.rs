use query_builder::{Query, CombinableQuery};

pub trait IntersectDsl<
    U: CombinableQuery,
    Type = <U as Query>::SqlType
>: CombinableQuery {
    type Output: Query<SqlType=Type>;

    fn intersect(self, query: U) -> Self::Output;
    fn intersect_all(self, query: U) -> Self::Output;
}
