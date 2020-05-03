use std::{env, io};
use blocks_world::methods::BlockMethod;
use blocks_world::pddl_parser::{make_block_problem_from, B};
use anyhop::BacktrackPreference::*;
use anyhop::BacktrackStrategy::*;
use anyhop::AnytimePlanner;
use blocks_world::operators::{BlockState, BlockGoals, BlockOperator};

fn main() -> io::Result<()> {
    for file in env::args().skip(1) {
        let (start, goal) = make_block_problem_from(file.as_str())?;
        for strategy in vec![Alternate(LeastRecent), Steady(LeastRecent), Steady(MostRecent)] {
            let outcome: AnytimePlanner<BlockState<B>, BlockGoals<B>, BlockOperator<B>, BlockMethod<B>, BlockMethod<B>, usize> = AnytimePlanner::plan(&start, &goal, None, strategy, &|p| p.len(), 1, true);
            println!("strategy: {:?}\n{}", strategy, outcome.report())
        }
    }
    Ok(())
}
