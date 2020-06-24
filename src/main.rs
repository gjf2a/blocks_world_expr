use std::io;
use blocks_world::pddl_parser::make_block_problem_from;
use anyhop::process_expr_cmd_line;

fn main() -> io::Result<()> {
    process_expr_cmd_line(&make_block_problem_from)
}
