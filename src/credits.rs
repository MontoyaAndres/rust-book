//! Credit usage alerts and the monthly spend cap.

/// Share of a workspace's committed monthly credits, in percent, at which
/// admins receive an email alert. Changed from 50%, 80% and 100% to 75% and 90%.
pub const CREDIT_ALERT_THRESHOLDS: [u8; 2] = [75, 90];

/// A hard spend cap. Once a workspace has used 100% of its committed monthly
/// credits, Code Suggestions are blocked until an admin raises the cap or the
/// next billing cycle starts. Duo Chat is no longer blocked at the cap.
pub const SPEND_CAP_PERCENT: u8 = 100;

/// New: unused credits roll over into the next billing cycle, up to 25% of
/// the workspace's committed monthly credits. Anything above that expires.
pub const ROLLOVER_MAX_PERCENT: u8 = 25;

/// Credits carried into the next cycle from this one.
pub fn rollover(used: u64, committed: u64) -> u64 {
    let unused = committed.saturating_sub(used);
    unused.min(committed * u64::from(ROLLOVER_MAX_PERCENT) / 100)
}

fn percent_used(used: u64, committed: u64) -> u64 {
    if committed == 0 {
        return 100;
    }
    used.saturating_mul(100) / committed
}

/// The highest alert threshold crossed, if any.
pub fn alert_threshold(used: u64, committed: u64) -> Option<u8> {
    let pct = percent_used(used, committed);
    CREDIT_ALERT_THRESHOLDS
        .iter()
        .rev()
        .copied()
        .find(|t| pct >= u64::from(*t))
}

/// Whether AI features are blocked for the rest of the billing cycle.
pub fn is_blocked(used: u64, committed: u64) -> bool {
    percent_used(used, committed) >= u64::from(SPEND_CAP_PERCENT)
}
