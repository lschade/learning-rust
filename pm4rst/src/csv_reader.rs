use std::collections::HashMap;

use crate::model::{Trace, Event};

pub fn read_csv(path: String) -> Vec<Trace> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut traces: HashMap<String, Trace> = HashMap::new();
    for row in rdr.deserialize() {
        if row.is_err() {
            println!("Error: {:?}", row.unwrap_err());
            continue;
        }
        let event: Event = row.unwrap();
        let trace = traces.entry(event.case_id.clone()).or_insert(Trace {
            case_id: event.case_id.clone(),
            events: Vec::new(),
        });
        trace.events.push(event);
    }

    let mut values: Vec<Trace> = traces.values().cloned().collect();
    values.sort_by(|a, b| a.case_id.cmp(&b.case_id));
    for trace in values.iter_mut() {
        trace.events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    }

    values
}