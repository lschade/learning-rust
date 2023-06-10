mod model;
mod csv_reader;
mod miner;

// test read_csv with running-example.csv as input
#[cfg(test)]
mod tests {
    use crate::csv_reader::read_csv;

    #[test]
    fn test_read_csv() {
        let traces = read_csv("running-example.csv".to_string());

        // print len of traces
        println!("len: {}", traces.len());
        assert!(traces.len() == 6);

        // is_sorted is still unstable: https://github.com/rust-lang/rust/issues/53485
        // traces.is_sorted_by_key(|trace| trace.case_id);
        // traces.iter().all(|trace| {
        //     trace.events.is_sorted_by_key(|event| event.timestamp)
        // });

        dbg!(traces);
    }
}
