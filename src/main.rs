fn main() {
    let input =
        std::fs::read_to_string("/home/gilescope/git/bumpotron/dry.txt").expect("file to exist");

    for line in input.lines() {
        if !line.starts_with("    Updating ") {
            continue;
        }
        println!("{}", line);
        let line = &line[13..];
        if let Some(pos) = line.find(' ') {
            let krate = &line[..pos];
            let mut update_cmd = std::process::Command::new("cargo");
            update_cmd.current_dir("/home/gilescope/git/substrate1/");
            update_cmd.args(vec!["update", "-p", krate]);
            println!("{:?}", update_cmd);

            let result = update_cmd.status();
            if result.is_ok() {
                //println!("{:?} done", update_cmd);
                // let mut clean_cmd = std::process::Command::new("cargo");
                // clean_cmd.current_dir("/home/gilescope/git/substrate1/frame/contracts");
                // clean_cmd.arg("clean").status().unwrap();

                let mut test_cmd = std::process::Command::new("cargo");
                test_cmd.current_dir("/home/gilescope/git/substrate1/frame/contracts");
                test_cmd.args(vec!["test", "self_destruct_works"]);
                let result = test_cmd.status();
                if let Ok(exit) = result {
                    if !exit.success() {
                        println!("not success");
                        break;
                    }
                }
                println!("success: {}", line);
            } else {
                break;
            }
        }
    }
}
