use std::str::FromStr;

use chrono::{DateTime, Utc};
use cron::Schedule;

use crate::{CronScheduleDef, CronScheduleKind};

pub fn compute_next_run(schedule: &CronScheduleDef, now_ms: i64) -> Option<i64> {
    match schedule.kind {
        CronScheduleKind::At => schedule.at_ms.filter(|ts| *ts > now_ms),
        CronScheduleKind::Every => schedule
            .every_ms
            .and_then(|ms| if ms <= 0 { None } else { Some(now_ms + ms) }),
        CronScheduleKind::Cron => {
            let expr = schedule.expr.as_deref()?;
            let parsed = Schedule::from_str(expr).ok()?;
            let now: DateTime<Utc> = Utc::now();
            parsed.after(&now).next().map(|dt| dt.timestamp_millis())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_next_run_every() {
        let now = crate::now_ms();
        let schedule = CronScheduleDef {
            kind: CronScheduleKind::Every,
            every_ms: Some(10_000),
            ..Default::default()
        };
        let next = compute_next_run(&schedule, now).unwrap();
        assert!(next >= now + 10_000);
    }

    #[test]
    fn compute_next_run_at_past_is_none() {
        let now = crate::now_ms();
        let schedule = CronScheduleDef {
            kind: CronScheduleKind::At,
            at_ms: Some(now - 1),
            ..Default::default()
        };
        assert!(compute_next_run(&schedule, now).is_none());
    }
}
