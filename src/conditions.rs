use sqlx::{Database, Encode, Type};

pub trait Conditions<'args, DB: Database> {

    /// Push always true (like 1 = 1) to the builder.
    fn push_always_true(&mut self) -> &mut Self;

    /// Push always false (like 1 != 1) to the builder.
    fn push_always_fasle(&mut self) -> &mut Self;

    /// Push ` <column_name> IN (<items...>) ` to the builder.
    /// If items are empty, push always false condition
    fn push_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self;

    /// Push ` AND <column_name> IN (<items...>) ` to the builder.
    /// If items are empty, push always false condition
    fn push_and_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self;

    /// Push ` OR <column_name> IN (<items...>) ` to the builder.
    /// If items are empty, push always false condition
    fn push_or_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self;

    /// Push ` <column_name> IN (<items...>) ` to the builder.
    /// If items are empty, push always true condition
    fn push_not_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self;

    /// Push ` AND <column_name> IN (<items...>) ` to the builder.
    /// If items are empty, push always true condition
    fn push_and_not_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self;

    /// Push ` OR <column_name> IN (<items...>) ` to the builder.
    /// If items are empty, push always true condition
    fn push_or_not_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self;
}
