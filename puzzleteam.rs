use core::panic;

use reqwest;

const PRINTOUTS: bool = false;

pub async fn load_puzzle(url: &str, size: i32) -> (String, String) {
    println!("Loading puzzle...");
    let client = reqwest::Client::new();
    let params = [
        ("new", "   New Puzzle   "),
        ("robot", "1"),
        ("b", "1"),
        ("size", "17"),
        ("w", &size.to_string()),
        ("h", &size.to_string()),
    ];

    let apitoken = "";

    let response = client
        .post(url)
        .form(&params)
        .header("Cookie", apitoken)
        .send()
        .await;

    let content: String;

    let task: String;
    let mut param: String;

    match response {
        Ok(r) => {
            content = r.text().await.unwrap();
            // This should be a constant to the puzzle
            let task_start = 18688;

            let task_end = match content.split_at(task_start).1.find("'") {
                Some(i) => task_start + i,
                None => panic!("no task end"),
            };

            task = match content.get(task_start..task_end) {
                Some(s) => s.to_string(),
                None => panic!("invalid task bounds"),
            };

            let param_start = task_end + 1934;
            let param_end = param_start + 320;

            param = match content.get(param_start..param_end) {
                Some(s) => s.to_string(),
                None => panic!("invalid param bounds"),
            };

            if param.contains('\"') {
                let param_start = match content.find("\"param") {
                    Some(i) => i + "\"param\" value=\"".len(),
                    None => panic!("no param found"),
                };

                let param_end = match content.split_at(param_start).1.find("\"") {
                    Some(i) => param_start + i,
                    None => panic!("no param end"),
                };

                param = match content.get(param_start..param_end) {
                    Some(s) => s.to_string(),
                    None => panic!("invalid param bounds"),
                };
            }
            if PRINTOUTS {
                println!("task start: {}, task end: {}", task_start, task_end);
                println!("param start: {}, param end: {}", param_start, param_end);
                println!("Puzzle loaded, param length {}", (param_end - param_start));
            }
        }
        Err(_) => {
            panic!("Error loading puzzle");
        }
    };

    (task, param)
}

pub async fn send_solution(param: &str, ans: &str) -> String {
    println!("Sending solution");
    let client = reqwest::Client::new();
    let params = [
        ("robot", "1"),
        ("param", &param),
        ("ansH", &ans),
        ("b", "1"),
        ("size", "17"),
        ("w", "10"),
        ("h", "10"),
        ("ready", "Done"),
    ];
    let apitoken = "api_token=";
    let response = client
        .post("https://www.puzzle-minesweeper.com/mosaic-10x10-easy/")
        .form(&params)
        .header("Cookie", apitoken)
        .send()
        .await;

    let mut solparam: String;
    match response {
        Ok(res) => {
            let content = res.text().await.unwrap();

            let param_start = 9714;
            let param_end = param_start + 236;

            solparam = match content.get(param_start..param_end) {
                Some(s) => s.to_string(),
                None => panic!("invalid solparam bounds"),
            };

            if solparam.contains('\"') {
                let param_start = match content.find("solparams") {
                    Some(i) => i + "solparams\" value=\"".len(),
                    None => panic!("no solparams start"),
                };

                let param_end = match content.split_at(param_start).1.find("\"") {
                    Some(i) => param_start + i,
                    None => panic!("no solparam end"),
                };
                solparam = match content.get(param_start..param_end) {
                    Some(s) => s.to_string(),
                    None => panic!("invalid solparams bounds"),
                };
            };

            if PRINTOUTS {
                println!(
                    "solparam start: {}, solparam end: {}",
                    param_start, param_end
                );
                println!(
                    "Solution sent, solparam length {}",
                    (param_end - param_start)
                );
            }
        }
        Err(_) => {
            panic!("Error sending solution");
        }
    };

    return solparam;
}

pub async fn submit_solution(solparam: &str) -> Result<reqwest::Response, reqwest::Error> {
    println!("Submitting time");
    let client = reqwest::Client::new();
    let params = [
        ("robot", "1"),
        ("solparams", &solparam),
        ("submitscore", "1"),
    ];
    let apitoken = "api_token=";
    let response = client
        .post("https://www.puzzle-minesweeper.com/hallsubmit.php")
        .form(&params)
        .header("Cookie", apitoken)
        .send()
        .await;

    match response {
        Ok(_) => println!("Time submitted"),
        Err(_) => println!("Error submitting time"),
    };
    response
}
