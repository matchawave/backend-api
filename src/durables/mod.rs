use worker::{Env, Stub};

pub mod bot;

pub trait DurableFetch {
    fn fetch_object(env: &Env, name: &str) -> worker::Result<Stub>;
}
