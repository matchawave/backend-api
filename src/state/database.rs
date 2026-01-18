use async_trait::async_trait;
use reqwest::StatusCode;
use sea_query::{
    DeleteStatement, InsertStatement, QueryStatement, QueryStatementWriter, SelectStatement,
    SqliteQueryBuilder, UpdateStatement, Value, Values,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tracing::error;
use wasm_bindgen::JsValue;
use worker::{D1Database, D1PreparedStatement, D1Result};

#[derive(Debug, Clone)]
pub struct Database(Arc<D1Database>);

impl From<D1Database> for Database {
    fn from(db: D1Database) -> Self {
        Self(db.into())
    }
}

#[async_trait(?Send)]
pub trait DatabaseExt<T, U> {
    async fn execute(&self, input: T) -> Result<U, StatusCode>;
    async fn batch(&self, inputs: Vec<T>) -> Result<Vec<U>, StatusCode>;
}

impl Database {
    pub fn new(database: D1Database) -> Self {
        Database(Arc::new(database))
    }

    fn build_query<Q: QueryStatementWriter>(
        &self,
        query: Q,
    ) -> Result<D1PreparedStatement, StatusCode> {
        let (query_str, params) = query.build(SqliteQueryBuilder);
        let params = convert_params(params);
        let instance = self.0.prepare(&query_str);
        match instance.bind(&params) {
            Ok(prepared) => Ok(prepared),
            Err(_) => {
                error!("Failed to bind parameters for query: {:?}", query_str);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn execute_run<Q: QueryStatementWriter>(&self, query: Q) -> Result<D1Result, StatusCode> {
        let instance = self.build_query(query)?;

        instance.run().await.map_err(|err| {
            error!("Database error: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }

    async fn batch_run<Q: QueryStatementWriter>(
        &self,
        queries: Vec<Q>,
    ) -> Result<Vec<D1Result>, StatusCode> {
        let mut statements = Vec::with_capacity(queries.len());

        for query in queries {
            let instance = self.build_query(query)?;
            statements.push(instance);
        }

        let results = self.0.batch(statements).await.map_err(|err| {
            error!("Database error during batch execution: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(results)
    }

    pub async fn batch_mixed(
        &self,
        queries: Vec<QueryStatement>,
    ) -> Result<Vec<serde_json::Value>, StatusCode> {
        let mut statements = Vec::with_capacity(queries.len());

        for query in queries {
            let q = match query {
                QueryStatement::Select(s) => self.build_query(s)?,
                QueryStatement::Insert(i) => self.build_query(i)?,
                QueryStatement::Update(u) => self.build_query(u)?,
                QueryStatement::Delete(d) => self.build_query(d)?,
            };
            statements.push(q);
        }

        let results = self.0.batch(statements).await.map_err(|err| {
            error!("Database error during batch execution: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let mut all_results = Vec::new();
        for result in results {
            let rows = result.results::<serde_json::Value>().map_err(|err| {
                error!("Database error while fetching batch results: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            all_results.push(serde_json::Value::Array(rows));
        }

        Ok(all_results)
    }
}

#[async_trait(?Send)]
impl DatabaseExt<InsertStatement, ()> for Database {
    async fn execute(&self, input: InsertStatement) -> Result<(), StatusCode> {
        self.execute_run(input).await?;
        Ok(())
    }

    async fn batch(&self, inputs: Vec<InsertStatement>) -> Result<Vec<()>, StatusCode> {
        let results = self.batch_run(inputs).await?;

        for result in results {
            result.results::<()>().map_err(|err| {
                tracing::error!("Database error while executing batch statement: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }

        Ok(Vec::new())
    }
}

#[async_trait(?Send)]
impl<T> DatabaseExt<SelectStatement, Vec<T>> for Database
where
    T: DeserializeOwned,
{
    async fn execute(&self, input: SelectStatement) -> Result<Vec<T>, StatusCode> {
        let result = self.execute_run(input).await?;

        result.results::<T>().map_err(|err| {
            error!("Database error while fetching results: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }

    async fn batch(&self, inputs: Vec<SelectStatement>) -> Result<Vec<Vec<T>>, StatusCode> {
        let results = self.batch_run(inputs).await?;

        let mut all_results = Vec::new();
        for result in results {
            let rows = result.results::<T>().map_err(|err| {
                tracing::error!("Database error while executing batch statement: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            all_results.push(rows);
        }

        Ok(all_results)
    }
}

#[async_trait(?Send)]
impl DatabaseExt<UpdateStatement, ()> for Database {
    async fn execute(&self, input: UpdateStatement) -> Result<(), StatusCode> {
        self.execute_run(input).await?;
        Ok(())
    }

    async fn batch(&self, inputs: Vec<UpdateStatement>) -> Result<Vec<()>, StatusCode> {
        let results = self.batch_run(inputs).await?;

        for result in results {
            result.results::<()>().map_err(|err| {
                tracing::error!("Database error while executing batch statement: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }

        Ok(Vec::new())
    }
}

#[async_trait(?Send)]
impl<T> DatabaseExt<DeleteStatement, Vec<T>> for Database
where
    T: DeserializeOwned,
{
    async fn execute(&self, input: DeleteStatement) -> Result<Vec<T>, StatusCode> {
        let result = self.execute_run(input).await?;

        result.results::<T>().map_err(|err| {
            error!("Database error while fetching deleted results: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
    }

    async fn batch(&self, inputs: Vec<DeleteStatement>) -> Result<Vec<Vec<T>>, StatusCode> {
        let results = self.batch_run(inputs).await?;

        let mut all_results = Vec::new();
        for result in results {
            let rows = result.results::<T>().map_err(|err| {
                tracing::error!("Database error while executing batch statement: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            all_results.push(rows);
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
