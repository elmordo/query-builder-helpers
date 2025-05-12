use sqlx::{Database, Encode, QueryBuilder, Type};
use std::marker::PhantomData;

#[derive(Debug, Default, Copy, Clone)]
pub struct Pagination<T, DB: Database>
where
    T: Type<DB> + for<'args> Encode<'args, DB>,
{
    pub limit: Option<T>,
    pub offset: Option<T>,
    _db_type: PhantomData<DB>,
}

impl<T, DB: Database> Pagination<T, DB>
where
    T: Type<DB> + for<'q> Encode<'q, DB>,
{
    pub fn new(limit: T, offset: T) -> Self {
        Self {
            limit: Some(limit),
            offset: Some(offset),
            _db_type: PhantomData,
        }
    }

    pub fn set_limit(&mut self, limit: T) -> &mut Self {
        self.limit = Some(limit);
        self
    }

    pub fn set_offset(&mut self, offset: T) -> &mut Self {
        self.offset = Some(offset);
        self
    }
}

pub trait PaginationExt<'args, DB: Database> {
    fn push_pagination<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        pagination: Pagination<T, DB>,
    ) -> &mut Self;
}

impl<'args, DB: Database> PaginationExt<'args, DB> for QueryBuilder<'args, DB> {
    fn push_pagination<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        pagination: Pagination<T, DB>,
    ) -> &mut Self {
        if let Some(offset) = pagination.offset {
            self.push(" OFFSET ");
            self.push_bind(offset);
            self.push(" ");
        }

        if let Some(limit) = pagination.limit {
            self.push(" LIMIT ");
            self.push_bind(limit);
            self.push(" ");
        }

        self
    }
}


#[cfg(feature = "postgres")]
pub type PgPagination = Pagination<i64, sqlx::Postgres>;
