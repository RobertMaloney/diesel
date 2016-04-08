use query_builder::{Query, CombinableQuery};

pub trait ExceptDsl<
    U: CombinableQuery,
    Type = <U as Query>::SqlType
>: CombinableQuery {
    type Output: Query<SqlType=Type>;

    fn except(self, query: U) -> Self::Output;
    fn except_all(self, query: U) -> Self::Output;
}
