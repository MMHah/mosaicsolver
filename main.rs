pub mod puzzleteam;

use futures::executor;
use mosaic_solver::*;
use tokio;

use std::time::Instant;

const PRINTOUTS: bool = false;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let (task, param): (String, String) = executor::block_on(puzzleteam::load_puzzle(
        "https://www.puzzle-minesweeper.com/mosaic-10x10-easy/",
        10,
    ));

    if PRINTOUTS {
        println!("task: {}", task);
        println!("param: {}", param);
    }

    let size: usize = 10;

    let solve_start = Instant::now();
    let (grid, solved, loop_count) = solve(read_task(&task, size), size);
    let solve_end = solve_start.elapsed();
    println!(
        "Solving took {} ms, with {} loops",
        solve_end.as_millis(),
        loop_count
    );

    if solved {
        let ans = parse_answer(&grid);
        let solparam = executor::block_on(puzzleteam::send_solution(&param, &ans));
        if PRINTOUTS {
            println!("\nSolved!");
            println!("solparam: {}", solparam);
        }
        let end = start.elapsed();
        println!("Total: {} ms", end.as_millis());
        let respo = puzzleteam::submit_solution(&solparam).await.unwrap();
        match respo.text().await {
            Ok(_) => println!("OK"),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("\nFailed to solve.");
        print_clues(&grid, size);
        print_result(&grid, size);
    }
}
