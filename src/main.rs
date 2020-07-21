extern crate log;
extern crate simple_logger;

use std::io;
use blocks_world::pddl_parser::make_block_problem_from;
use anyhop::{process_expr_cmd_line, CmdArgs};

fn main() -> io::Result<()> {
    simple_logger::init().unwrap();
    let args = CmdArgs::new()?;
    process_expr_cmd_line(&make_block_problem_from, &args)
}
