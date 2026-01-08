use reqwest::StatusCode;
use sea_query::{Value, Values};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tracing::error;
use wasm_bindgen::JsValue;
use worker::{D1Database, D1PreparedStatement};

#[derive(Debug, Clone)]
pub struct Databases {
    pub general: Database,
    pub levels: Database,
}

#[derive(Debug, Clone)]
pub struct Database(Arc<D1Database>);

impl Database {
    pub fn new(database: D1Database) -> Self {
        Database(Arc::new(database))
    }

    pub async fn select<T>(&self, query: (String, Values)) -> Result<Vec<T>, StatusCode>
    where
        T: DeserializeOwned,
    {
        let instance = build_query(&self.0, query)?;
        let results = instance.run().await.and_then(|r| r.results::<T>());
        match results {
            Ok(res) => Ok(res),
            Err(err) => {
                error!("Database error: {}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    pub async fn insert(&self, query: (String, Values)) -> Result<(), StatusCode> {
        let instance = build_query(&self.0, query)?;

        instance.run().await.map_err(|err| {
            error!("Database error: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Ok(())
    }
}

fn build_query(
    database: &Arc<D1Database>,
    query: (String, Values),
) -> Result<D1PreparedStatement, StatusCode> {
    let (query_str, params) = query;
    let params = convert_params(params)?;
    let instance = database.prepare(&query_str);
    match instance.bind(&params) {
        Ok(prepared) => Ok(prepared),
        Err(_) => {
            error!("Failed to bind parameters for query: {:?}", query_str);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn convert_params(values: Values) -> Result<Vec<JsValue>, StatusCode> {
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

    Ok(params)
}
