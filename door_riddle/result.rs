use std::iter::Take;
use std::sync::mpsc::Iter;

#[derive(Default)]
pub struct Result {
    pub failed: u64,
    pub successful: u64,
}

impl Result {
    pub fn combine(results: Take<Iter<Result>>) -> Self {
        let mut complete_result = Result::default();
        for result in results {
            complete_result.failed += result.failed;
            complete_result.successful += result.successful;
        }
        return complete_result;
    }
}
