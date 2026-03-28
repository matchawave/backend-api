use async_trait::async_trait;
use sea_query::{
    DeleteStatement, InsertStatement, QueryStatement, QueryStatementWriter, SelectStatement,
    SqliteQueryBuilder, UpdateStatement, Value, Values,
};
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;
use worker::{D1Database, D1PreparedStatement, D1Result, Env, send::SendWrapper};

#[derive(Debug, Clone)]
pub struct MixedResult<S, I, U, D>
where
    S: DeserializeOwned,
    I: DeserializeOwned,
    U: DeserializeOwned,
    D: DeserializeOwned,
{
    pub select: Option<Vec<S>>,
    pub insert: Option<Vec<I>>,
    pub update: Option<Vec<U>>,
    pub delete: Option<Vec<D>>,
}

impl<S, I, U, D> Default for MixedResult<S, I, U, D>
where
    S: DeserializeOwned,
    I: DeserializeOwned,
    U: DeserializeOwned,
    D: DeserializeOwned,
{
    fn default() -> Self {
        Self {
            select: None,
            insert: None,
            update: None,
            delete: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    env: SendWrapper<Env>,
    binding: String,
}

#[async_trait(?Send)]
pub trait DatabaseExt<Q, U>
where
    Q: QueryStatementWriter,
    U: DeserializeOwned,
{
    async fn execute(&self, input: Q) -> worker::Result<U>;
    async fn batch(&self, inputs: &[Q]) -> worker::Result<U>;
}

impl Database {
    pub fn new<T: Into<String>>(env: &Env, binding: T) -> Self {
        Self {
            env: SendWrapper::new(env.clone()),
            binding: binding.into(),
        }
    }

    fn get_db(&self) -> worker::Result<D1Database> {
        self.env.d1(&self.binding)
    }

    fn build_query<Q: QueryStatementWriter>(
        &self,
        query: Q,
    ) -> worker::Result<D1PreparedStatement> {
        let (query_str, params) = query.build(SqliteQueryBuilder);
        let params = convert_params(params);
        let instance = self.get_db()?.prepare(&query_str);
        instance.bind(&params)
    }

    async fn execute_run<Q: QueryStatementWriter>(&self, query: Q) -> worker::Result<D1Result> {
        let instance = self.build_query(query)?;
        instance.run().await
    }

    async fn batch_run<Q: QueryStatementWriter + Clone>(
        &self,
        queries: &[Q],
    ) -> worker::Result<Vec<D1Result>> {
        let mut statements = Vec::with_capacity(queries.len());

        for query in queries.iter().cloned() {
            let instance = self.build_query(query)?;
            statements.push(instance);
        }

        self.get_db()?.batch(statements).await
    }

    pub async fn batch_mixed<S, I, U, D>(
        &self,
        queries: &[QueryStatement],
    ) -> worker::Result<MixedResult<S, I, U, D>>
    where
        S: DeserializeOwned,
        I: DeserializeOwned,
        U: DeserializeOwned,
        D: DeserializeOwned,
    {
        let mut statements = Vec::with_capacity(queries.len());

        for query in queries.iter().cloned() {
            let statement = match query {
                QueryStatement::Select(s) => self.build_query(s)?,
                QueryStatement::Insert(i) => self.build_query(i)?,
                QueryStatement::Update(u) => self.build_query(u)?,
                QueryStatement::Delete(d) => self.build_query(d)?,
            };
            statements.push(statement);
        }
        let results = self.get_db()?.batch(statements).await?;

        let mut result_output = MixedResult::default();
        for (i, result) in results.iter().enumerate() {
            match queries.get(i) {
                Some(QueryStatement::Select(_)) => {
                    let rows = result.results::<S>()?;
                    result_output.select = Some(rows);
                }
                Some(QueryStatement::Insert(_)) => {
                    let rows = result.results::<I>()?;
                    result_output.insert = Some(rows);
                }
                Some(QueryStatement::Update(_)) => {
                    let rows = result.results::<U>()?;
                    result_output.update = Some(rows);
                }
                Some(QueryStatement::Delete(_)) => {
                    let rows = result.results::<D>()?;
                    result_output.delete = Some(rows);
                }
                None => return Err(worker::Error::from("Unexpected query type")),
            }
        }

        Ok(result_output)
    }
}

#[async_trait(?Send)]
impl DatabaseExt<InsertStatement, ()> for Database {
    async fn execute(&self, input: InsertStatement) -> worker::Result<()> {
        self.execute_run(input).await?;
        Ok(())
    }

    async fn batch(&self, inputs: &[InsertStatement]) -> worker::Result<()> {
        let results = self.batch_run(inputs).await?;

        for result in results {
            result.results::<()>()?;
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl<T> DatabaseExt<InsertStatement, Vec<T>> for Database
where
    T: DeserializeOwned,
{
    async fn execute(&self, input: InsertStatement) -> worker::Result<Vec<T>> {
        let result = self.execute_run(input).await?;
        result.results::<T>()
    }

    async fn batch(&self, inputs: &[InsertStatement]) -> worker::Result<Vec<T>> {
        let results = self.batch_run(inputs).await?;

        let mut all_results = Vec::new();
        for result in results {
            let mut rows = result.results::<T>()?;
            all_results.append(&mut rows);
        }

        Ok(all_results)
    }
}

#[async_trait(?Send)]
impl<T> DatabaseExt<SelectStatement, Vec<T>> for Database
where
    T: DeserializeOwned,
{
    async fn execute(&self, input: SelectStatement) -> worker::Result<Vec<T>> {
        let result = self.execute_run(input).await?;
        result.results::<T>()
    }

    async fn batch(&self, inputs: &[SelectStatement]) -> worker::Result<Vec<T>> {
        let results = self.batch_run(inputs).await?;

        let mut all_results = Vec::new();
        for result in results {
            let mut rows = result.results::<T>()?;
            all_results.append(&mut rows);
        }

        Ok(all_results)
    }
}

#[async_trait(?Send)]
impl DatabaseExt<UpdateStatement, ()> for Database {
    async fn execute(&self, input: UpdateStatement) -> worker::Result<()> {
        self.execute_run(input).await?;
        Ok(())
    }

    async fn batch(&self, inputs: &[UpdateStatement]) -> worker::Result<()> {
        let results = self.batch_run(inputs).await?;

        for result in results {
            result.results::<()>()?;
        }

        Ok(())
    }
}

#[async_trait(?Send)]
impl<T> DatabaseExt<DeleteStatement, Vec<T>> for Database
where
    T: DeserializeOwned,
{
    async fn execute(&self, input: DeleteStatement) -> worker::Result<Vec<T>> {
        let result = self.execute_run(input).await?;
        result.results::<T>()
    }

    async fn batch(&self, inputs: &[DeleteStatement]) -> worker::Result<Vec<T>> {
        let results = self.batch_run(inputs).await?;

        let mut all_results = Vec::new();
        for result in results {
            let mut rows = result.results::<T>()?;
            all_results.append(&mut rows);
        }

        Ok(all_results)
    }
}

fn convert_params(values: Values) -> Vec<JsValue> {
    let values = values.0;
    let mut params: Vec<JsValue> = Vec::with_capacity(values.len());

    for v in values {
        let value = match v {
            Value::Bool(Some(b)) => JsValue::from_bool(b),
            Value::Char(Some(c)) => JsValue::from_str(&c.to_string()),
            Value::String(Some(s)) => JsValue::from_str(&(*s).clone()),
            // Signed number types
            Value::TinyInt(Some(i)) => JsValue::from_f64(i as f64),
            Value::SmallInt(Some(i)) => JsValue::from_f64(i as f64),
            Value::Int(Some(i)) => JsValue::from_f64(i as f64),
            Value::BigInt(Some(i)) => JsValue::from_f64(i as f64),
            // Unsigned number types
            Value::TinyUnsigned(Some(u)) => JsValue::from_f64(u as f64),
            Value::SmallUnsigned(Some(u)) => JsValue::from_f64(u as f64),
            Value::Unsigned(Some(u)) => JsValue::from_f64(u as f64),
            Value::BigUnsigned(Some(u)) => JsValue::from_f64(u as f64),
            // Float types
            Value::Float(Some(f)) => JsValue::from_f64(f as f64),
            Value::Double(Some(f)) => JsValue::from_f64(f),
            // Handle NULL values for all types
            _ => JsValue::NULL,
        };
        params.push(value);
    }

    params
}

pub trait IntoQueryStatement {
    fn into_query_statement(self) -> QueryStatement;
}

impl IntoQueryStatement for InsertStatement {
    fn into_query_statement(self) -> QueryStatement {
        QueryStatement::Insert(self)
    }
}

impl IntoQueryStatement for SelectStatement {
    fn into_query_statement(self) -> QueryStatement {
        QueryStatement::Select(self)
    }
}

impl IntoQueryStatement for UpdateStatement {
    fn into_query_statement(self) -> QueryStatement {
        QueryStatement::Update(self)
    }
}

impl IntoQueryStatement for DeleteStatement {
    fn into_query_statement(self) -> QueryStatement {
        QueryStatement::Delete(self)
    }
}

pub struct QueryBuilder(Vec<QueryStatement>);

impl QueryBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(mut self, stmt: impl IntoQueryStatement) -> Self {
        self.0.push(stmt.into_query_statement());
        self
    }
}

impl From<QueryBuilder> for Vec<QueryStatement> {
    fn from(builder: QueryBuilder) -> Self {
        builder.0
    }
}

impl From<QueryBuilder> for Vec<InsertStatement> {
    fn from(builder: QueryBuilder) -> Self {
        builder
            .0
            .into_iter()
            .filter_map(|stmt| {
                if let QueryStatement::Insert(insert) = stmt {
                    Some(insert)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<QueryBuilder> for Vec<SelectStatement> {
    fn from(builder: QueryBuilder) -> Self {
        builder
            .0
            .into_iter()
            .filter_map(|stmt| {
                if let QueryStatement::Select(select) = stmt {
                    Some(select)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<QueryBuilder> for Vec<UpdateStatement> {
    fn from(builder: QueryBuilder) -> Self {
        builder
            .0
            .into_iter()
            .filter_map(|stmt| {
                if let QueryStatement::Update(update) = stmt {
                    Some(update)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<QueryBuilder> for Vec<DeleteStatement> {
    fn from(builder: QueryBuilder) -> Self {
        builder
            .0
            .into_iter()
            .filter_map(|stmt| {
                if let QueryStatement::Delete(delete) = stmt {
                    Some(delete)
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Macro to create a Vec<QueryStatement> from mixed statement types
/// Usage: queries![insert_stmt, select_stmt, update_stmt, delete_stmt]
/// Macro that creates a QueryBuilder which can convert to different vector types
#[macro_export]
macro_rules! queries {
    () => {
        $crate::state::database::QueryBuilder::new()
    };
    ($($stmt:expr),+ $(,)?) => {
        {
            let mut builder = $crate::state::database::QueryBuilder::new();
            $(
                builder = builder.add($stmt);
            )+
            builder.into()
        }
    };
}
