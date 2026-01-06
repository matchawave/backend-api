use reqwest::StatusCode;
use sea_query::{Value, Values};
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;
use wasm_bindgen::JsValue;
use worker::{console_debug, console_log, D1Database, D1Result};

#[derive(Debug, Clone)]
pub struct Databases {
    pub general: Database,
    pub levels: Database,
}

#[derive(Debug, Clone)]
pub struct Database {
    database: Arc<D1Database>,
}

impl Database {
    pub fn new(database: D1Database) -> Self {
        Database {
            database: Arc::new(database),
        }
    }
    pub async fn exec_returning<T>(&self, query: (String, Values)) -> Result<Vec<T>, StatusCode>
    where
        T: for<'a> Deserialize<'a>,
    {
        let (query_str, params) = query;
        let params = convert_params(params)?;
        let instance = self.database.prepare(&query_str);
        let instance = instance.bind(&params).map_err(|_| {
            error!("Failed to bind parameters for query: {:?}", query_str);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        instance
            .run()
            .await
            .and_then(|result| match result.results::<T>() {
                Ok(res) => Ok(res),
                Err(e) => Err(e),
            })
            .map_err(|err| {
                error!("Database error: {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }

    pub async fn exec(&self, query: (String, Values)) -> Result<(), StatusCode> {
        let (query_str, params) = query;
        let params = convert_params(params)?;
        let instance = self.database.prepare(&query_str);
        let instance = instance.bind(&params).map_err(|_| {
            error!("Failed to bind parameters for query: {:?}", query_str);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        instance.run().await.map_err(|err| {
            error!("Database error: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        Ok(())
    }
}

fn convert_params(values: Values) -> Result<Vec<JsValue>, StatusCode> {
    let values = values.0;
    let mut params: Vec<JsValue> = Vec::with_capacity(values.len());

    for v in values {
        match v {
            Value::Bool(Some(b)) => params.push(JsValue::from_bool(b)),
            Value::Int(Some(i)) => params.push(JsValue::from_f64(i as f64)),
            Value::TinyInt(Some(i)) => params.push(JsValue::from_f64(i as f64)),
            Value::SmallInt(Some(i)) => params.push(JsValue::from_f64(i as f64)),
            Value::BigInt(Some(i)) => params.push(JsValue::from_f64(i as f64)),
            Value::Char(Some(c)) => params.push(JsValue::from_str(&c.to_string())),
            Value::Double(Some(f)) => params.push(JsValue::from_f64(f)),
            Value::Float(Some(f)) => params.push(JsValue::from_f64(f as f64)),
            Value::String(Some(s)) => params.push(JsValue::from_str(&(*s).clone())),

            _ => {
                error!("Unsupported or NULL parameter");
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    Ok(params)
}
