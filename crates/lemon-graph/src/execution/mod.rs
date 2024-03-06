mod step;

use petgraph::graph::NodeIndex;
pub use step::*;

use crate::Graph;

pub struct Executor;

impl Executor {
    pub async fn execute(graph: &mut Graph, start: NodeIndex) -> Result<(), ExecutionStepError> {
        let mut steps = vec![ExecutionStep(start)];

        while let Some(step) = steps.pop() {
            let next_steps = step.execute(graph).await?;
            steps.extend(next_steps);
        }

        Ok(())
    }
}
