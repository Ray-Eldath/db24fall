use std::sync::atomic::{AtomicU64, AtomicUsize};

#[derive(Debug)]
pub(crate) struct Stats {
    pub(crate) articles_count: AtomicUsize,
    pub(crate) refs_before_filtering: AtomicUsize,
    pub(crate) refs_after_filtering: AtomicUsize,
    pub(crate) jobs_range: (AtomicUsize, AtomicUsize), // start, count. [start, start + count - 1]
    pub(crate) completed_job: AtomicUsize,
}

pub(crate) static STATS: Stats = Stats::new();

impl Stats {
    const fn new() -> Self {
        Stats {
            articles_count: AtomicUsize::new(0),
            refs_before_filtering: AtomicUsize::new(0),
            refs_after_filtering: AtomicUsize::new(0),
            jobs_range: (AtomicUsize::new(0), AtomicUsize::new(0)),
            completed_job: AtomicUsize::new(0),
        }
    }
}
