use sqlx::{Database, Encode, QueryBuilder, Type};

pub trait Conditions<'args, DB: Database> {
    /// Push always true (like 1 = 1) to the builder.
    fn push_always_true(&mut self) -> &mut Self;

    /// Push always false (like 1 != 1) to the builder.
    fn push_always_false(&mut self) -> &mut Self;

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

    /// Bind many items separated by `separator`
    fn push_bind_many<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        values: impl IntoIterator<Item = T>,
        separator: char,
    ) -> &mut Self;

    /// Push the '('
    fn push_open_parenthesis(&mut self) -> &mut Self;

    /// Push the ')'
    fn push_close_parenthesis(&mut self) -> &mut Self;
}

impl<'args, DB: Database> Conditions<'args, DB> for QueryBuilder<'args, DB> {
    fn push_always_true(&mut self) -> &mut Self {
        self.push(" 1 = 1 ")
    }

    fn push_always_false(&mut self) -> &mut Self {
        self.push(" 1 != 1 ")
    }

    fn push_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        let items = items.into_iter().collect::<Vec<_>>();
        if items.is_empty() {
            self.push_always_false()
        } else {
            self.push(&format!(" {} IN ", column_name))
                .push_open_parenthesis()
                .push_bind_many(items, ',')
                .push_close_parenthesis()
        }
    }

    fn push_and_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        self.push(" AND ").push_in_collection(column_name, items)
    }

    fn push_or_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        self.push(" OR ").push_in_collection(column_name, items)
    }

    fn push_not_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        let items = items.into_iter().collect::<Vec<_>>();
        if items.is_empty() {
            self.push_always_true()
        } else {
            self.push(&format!(" {} NOT IN ", column_name))
                .push_open_parenthesis()
                .push_bind_many(items, ',')
                .push_close_parenthesis()
        }
    }

    fn push_and_not_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        self.push(" AND ").push_not_in_collection(column_name, items)
    }

    fn push_or_not_in_collection<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        column_name: &str,
        items: impl IntoIterator<Item = T>,
    ) -> &mut Self {
        self.push(" OR ").push_not_in_collection(column_name, items)
    }

    /// Bind many items separated by `separator`
    fn push_bind_many<T: 'args + Type<DB> + for<'q> Encode<'q, DB>>(
        &mut self,
        values: impl IntoIterator<Item = T>,
        separator: char,
    ) -> &mut Self {
        let mut sep = self.separated(separator);
        for item in values {
            sep.push_bind(item);
        }
        self
    }

    /// Push the '('
    fn push_open_parenthesis(&mut self) -> &mut Self {
        self.push(" ( ")
    }

    /// Push the ')'
    fn push_close_parenthesis(&mut self) -> &mut Self {
        self.push(" ) ")
    }
}
