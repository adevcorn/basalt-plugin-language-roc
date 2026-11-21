use basalt_roc_semantic_facts::extract_facts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyLane {
    InMemoryWasm32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyPlan {
    pub lane: VerifyLane,
    pub pure_targets: Vec<String>,
    pub mocked_task_targets: Vec<String>,
}

pub fn plan_verification(source: &str) -> VerifyPlan {
    let facts = extract_facts(source);
    VerifyPlan {
        lane: VerifyLane::InMemoryWasm32,
        pure_targets: facts.pure_functions.into_iter().map(|s| s.name).collect(),
        mocked_task_targets: facts.task_functions.into_iter().map(|s| s.name).collect(),
    }
}
