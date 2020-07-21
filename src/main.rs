use std::io;
use blocks_world::pddl_parser::make_block_problem_from;
use anyhop::{process_expr_cmd_line, CmdArgs};

fn main() -> io::Result<()> {
    let args = CmdArgs::new()?;
    process_expr_cmd_line(&make_block_problem_from, &args)
}
