use crate::config::CONFIG;
use crate::prelude::*;
use std::process::{Command, Stdio};

static VERSION_DISCLAIMER: &str =
    "The cheat client local detail and install from https://github.com/cheat/cheat)
 you test cheat commad you config is god $cheat --path=<name> <cheat>";

//--- from Cheatsh.rc
fn map_line(line: &str) -> String {
    line.trim().trim_end_matches(':').to_string()
}

fn as_lines(query: &str, markdown: &str) -> Vec<String> {
    format!(
        "% {query}, cheat
{markdown}"
    )
    .lines()
    .map(map_line)
    .collect()
}

//---

pub fn call(query: &str) -> Result<Vec<String>> {
    let cheat_cfg = CONFIG.cheat();
    let cheat_opt = if !cheat_cfg.is_empty() { &cheat_cfg } else { "--path=community" };
    //eprintln!("Debug: cheat_cfg={:?} cheat_opy={}", cheat_opt, cheat_opt);
    let args = [cheat_opt, query];

    let child = Command::new("cheat")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let child = match child {
        Ok(x) => x,
        Err(_) => {
            let msg = format!(
                "navi was unable to call tldr.
Make sure tldr is correctly installed.

Note:
{VERSION_DISCLAIMER}
"
            );
            return Err(anyhow!(msg));
        }
    };

    let out = child.wait_with_output().context("Failed to wait for tldr")?;

    if let Some(0) = out.status.code() {
        let stdout = out.stdout;
        let markdown = String::from_utf8(stdout).context("Output is invalid utf8")?;
        let cheat_cmd = format!("# **Run command 'cheat {cheat_opt} {query}' to show cheat** \n cheat {cheat_opt} {query} \n #" ) ;
        let markdown = format!("{}{}", cheat_cmd, markdown);
        // eprintln!("Debug: markdown = {}", markdown);
        let lines = as_lines(query, &markdown);
        Ok(lines)
    } else {
        let msg = format!(
            "Failed to call:
cheat {}

Output:
{}

Error:
{}

Note:
The cheat client local detail and install from https://github.com/cheat/cheat)
config option --path=<nae> or --tags=<tag>. you caht test opt $cheat --path=<name> <cheat>
If you are already using a supported version you can ignore this message.
{}
",
            args.join(" "),
            String::from_utf8(out.stdout).unwrap_or_else(|_e| "Unable to get output message".to_string()),
            String::from_utf8(out.stderr).unwrap_or_else(|_e| "Unable to get error message".to_string()),
            VERSION_DISCLAIMER,
        );
        Err(anyhow!(msg))
    }
}
