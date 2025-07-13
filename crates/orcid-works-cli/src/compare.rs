use std::collections::{HashMap, HashSet};

use orcid_works_model::{OrcidWorkDetail, OrcidWorks};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diff {
    Added,
    Updated,
    Kept,
    Deleted,
}

pub fn diff_putcodes(
    older: &HashMap<u64, OrcidWorkDetail>,
    newer: &OrcidWorks,
    force_fetch: bool,
) -> HashMap<u64, Diff> {
    let mut diff = HashMap::new();

    let mut seen_old: HashSet<u64> = HashSet::new();

    for g in &newer.group {
        for s in &g.work_summary {
            let pc = s.put_code;
            let newt = s.last_modified_date.value;

            match older.get(&pc) {
                None => {
                    diff.insert(pc, Diff::Added);
                }
                Some(t) => {
                    seen_old.insert(pc);
                    let oldt = t.summary.last_modified_date.value;
                    if newt > oldt || force_fetch {
                        diff.insert(pc, Diff::Updated);
                    } else {
                        diff.insert(pc, Diff::Kept);
                    }
                }
            }
        }
    }

    for &pc in older.keys() {
        if !seen_old.contains(&pc) {
            diff.insert(pc, Diff::Deleted);
        }
    }

    diff
}

pub fn added_putcodes(diff: &HashMap<u64, Diff>) -> Vec<u64> {
    diff.iter()
        .filter_map(|(&pc, &d)| (d == Diff::Added).then_some(pc))
        .collect()
}

pub fn updated_putcodes(diff: &HashMap<u64, Diff>) -> Vec<u64> {
    diff.iter()
        .filter_map(|(&pc, &d)| (d == Diff::Updated).then_some(pc))
        .collect()
}

pub fn kept_putcodes(diff: &HashMap<u64, Diff>) -> Vec<u64> {
    diff.iter()
        .filter_map(|(&pc, &d)| (d == Diff::Kept).then_some(pc))
        .collect()
}

pub fn deleted_putcodes(diff: &HashMap<u64, Diff>) -> Vec<u64> {
    diff.iter()
        .filter_map(|(&pc, &d)| (d == Diff::Deleted).then_some(pc))
        .collect()
}
