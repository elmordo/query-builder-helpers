use sqlx::{Database, Encode, QueryBuilder, Type};
use std::marker::PhantomData;

#[derive(Debug, Default, Copy, Clone)]
pub struct PaginationSettings<T, DB: Database>
where
    T: Type<DB> + for<'args> Encode<'args, DB>,
{
    pub limit: Option<T>,
    pub offset: Option<T>,
    _db_type: PhantomData<DB>,
}

impl<T, DB: Database> PaginationSettings<T, DB>
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

pub trait Pagination<'args, DB: Database> {
    fn push_pagination<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        pagination: PaginationSettings<T, DB>,
    ) -> &mut Self;

    fn push_optional_pagination<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        pagination: Option<PaginationSettings<T, DB>>,
    ) -> &mut Self {
        if let Some(pagination) = pagination {
            self.push_pagination(pagination)
        } else {
            self
        }
    }
}

impl<'args, DB: Database> Pagination<'args, DB> for QueryBuilder<'args, DB> {
    fn push_pagination<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        pagination: PaginationSettings<T, DB>,
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
