use anyhow::Result;
use ferrumbot_cron::{CronPayload, CronScheduleDef, CronScheduleKind, CronService};
use serde_json::Value;

pub(super) async fn handle(action: &str, args: &Value, cron: &CronService) -> Result<String> {
    match action {
        "list" => list_jobs(cron).await,
        "remove" => remove_job(args, cron).await,
        "add" => add_job(args, cron).await,
        _ => Ok(format!("Error: unsupported action {action}")),
    }
}

async fn list_jobs(cron: &CronService) -> Result<String> {
    let jobs = cron.list_jobs(true).await;
    if jobs.is_empty() {
        return Ok("No scheduled jobs.".to_string());
    }

    let rows = jobs
        .iter()
        .map(|job| {
            format!(
                "{} | {} | {:?} | enabled={}",
                job.id, job.name, job.schedule.kind, job.enabled
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    Ok(rows)
}

async fn remove_job(args: &Value, cron: &CronService) -> Result<String> {
    let id = args.get("id").and_then(|v| v.as_str()).unwrap_or_default();
    let removed = cron.remove_job(id).await?;
    if removed {
        Ok(format!("Removed job {id}"))
    } else {
        Ok(format!("Job {id} not found"))
    }
}

async fn add_job(args: &Value, cron: &CronService) -> Result<String> {
    let name = args
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("job")
        .to_string();
    let message = args
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
    let schedule = schedule_from_args(args);

    let payload = CronPayload {
        kind: "agent_turn".to_string(),
        message,
        deliver: false,
        channel: None,
        to: None,
    };

    let job = cron.add_job(name, schedule, payload, false).await?;
    Ok(format!("Added job '{}' ({})", job.name, job.id))
}

fn schedule_from_args(args: &Value) -> CronScheduleDef {
    if let Some(every) = args.get("every").and_then(|v| v.as_i64()) {
        return CronScheduleDef {
            kind: CronScheduleKind::Every,
            every_ms: Some(every * 1000),
            ..Default::default()
        };
    }

    if let Some(expr) = args.get("cron").and_then(|v| v.as_str()) {
        return CronScheduleDef {
            kind: CronScheduleKind::Cron,
            expr: Some(expr.to_string()),
            ..Default::default()
        };
    }

    let at = args.get("at").and_then(|v| v.as_i64()).unwrap_or_default();
    CronScheduleDef {
        kind: CronScheduleKind::At,
        at_ms: Some(at),
        ..Default::default()
    }
}
