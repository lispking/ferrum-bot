use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::Result;
use futures::future::BoxFuture;

use crate::CronJob;

pub type JobCallback =
    Arc<dyn Fn(CronJob) -> BoxFuture<'static, Result<Option<String>>> + Send + Sync>;

pub fn boxed_callback<F, Fut>(f: F) -> JobCallback
where
    F: Fn(CronJob) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Option<String>>> + Send + 'static,
{
    Arc::new(move |job| {
        let fut = f(job);
        Box::pin(fut) as Pin<Box<dyn Future<Output = Result<Option<String>>> + Send>>
    })
}
