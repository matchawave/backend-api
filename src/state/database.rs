use reqwest::StatusCode;
use sea_query::{QueryStatementWriter, SqliteQueryBuilder, Value, Values};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tracing::error;
use wasm_bindgen::JsValue;
use worker::{D1Database, D1PreparedStatement};

#[derive(Debug, Clone)]
pub struct Database(Arc<D1Database>);

impl From<D1Database> for Database {
    fn from(db: D1Database) -> Self {
        Self(db.into())
    }
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

    pub async fn select<T, Q>(&self, query: Q) -> Result<Vec<T>, StatusCode>
    where
        T: DeserializeOwned,
        Q: QueryStatementWriter,
    {
        let instance = self.build_query(query)?;
        let results = instance.run().await.and_then(|r| r.results::<T>());
        match results {
            Ok(res) => Ok(res),
            Err(err) => {
                error!("Database error: {}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub async fn select_one<T, Q>(&self, query: Q) -> Result<Option<T>, StatusCode>
    where
        T: DeserializeOwned,
        Q: QueryStatementWriter,
    {
        let results = self.select::<T, Q>(query).await?;
        if results.is_empty() {
            return Ok(None);
        }
        if results.len() > 1 {
            error!("Expected one result, found multiple");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(results.into_iter().next())
    }

    pub async fn insert<Q>(&self, query: Q) -> Result<(), StatusCode>
    where
        Q: QueryStatementWriter,
    {
        let instance = self.build_query(query)?;

        instance.run().await.map_err(|err| {
            error!("Database error: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Ok(())
    }

    pub async fn batch<T>(&self, queries: Vec<(String, Values)>) -> Result<Vec<Vec<T>>, StatusCode>
    where
        T: DeserializeOwned,
    {
        let mut statements = Vec::with_capacity(queries.len());

        for query in queries {
            let instance = get_query(&self.0, query)?;
            statements.push(instance);
        }

        let results = self.0.batch(statements).await.map_err(|err| {
            error!("Database error during batch execution: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let mut all_results = Vec::new();
        for result in results {
            let rows: Vec<T> = result.results::<T>().map_err(|err| {
                error!("Database error while fetching batch results: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            all_results.push(rows);
        }

        Ok(all_results)
    }

    pub async fn batch_mixed(
        &self,
        queries: Vec<(String, Values)>,
    ) -> Result<Vec<serde_json::Value>, StatusCode> {
        let mut statements = Vec::with_capacity(queries.len());

        for query in queries {
            let instance = get_query(&self.0, query)?;
            statements.push(instance);
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
            _ => {
                tracing::warn!(
                    "{:?} if a invalid or NULL value was provided for a query parameter",
                    v
                );
                JsValue::NULL
            }
        };
        params.push(value);
    }

    params
}
