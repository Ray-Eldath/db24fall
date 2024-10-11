mod de;
mod ser;
mod stats;

use rayon::prelude::*;

use crate::ser::run_de_ser;
use crate::stats::STATS;
use std::fs;
use std::io::{LineWriter, Write};
use std::sync::atomic::Ordering;

fn do_deser(prefix: String) {
    let xml_path = format!("{}.xml", prefix);
    let content = fs::read_to_string(&xml_path).unwrap();
    let deser = run_de_ser(&content);

    let file = fs::File::create(format!("{}.ndjson", prefix)).unwrap();
    let mut file = LineWriter::new(file);
    for str in deser.iter() {
        file.write_all(str.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }

    STATS.articles_count.fetch_add(deser.len(), Ordering::SeqCst);
    println!(
        "[{: >3}] ({: >3} / {: >3}) {}: {}",
        rayon::current_thread_index().unwrap(),
        STATS.completed_job.fetch_add(1, Ordering::SeqCst) + 1,
        STATS.jobs_range.1.load(Ordering::SeqCst),
        &xml_path,
        deser.len()
    );
}

fn main() {
    let starts_from = 1;
    let count = 100;
    STATS.jobs_range.0.store(starts_from, Ordering::SeqCst);
    STATS.jobs_range.1.store(count, Ordering::SeqCst);
    let basepath = r"C:\Users\ray-eldath\Downloads\pubmed-2024\pubmed24n";
    (starts_from..=(starts_from + count - 1))
        .into_par_iter()
        .for_each(|i| do_deser(format!("{}{:0>4}", basepath, i)));
    println!("{:#?}", STATS);
}
