use std::io::{self, Write};
use std::mem;
use std::process::Command;

/// Extract bash code blocks and execute them
pub fn extract_and_execute_commands(
    response: &str,
    auto_approve: bool,
    allow_unsafe_exec: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if auto_approve && !allow_unsafe_exec {
        eprintln!("\n[ai-coder-agent] ⚠️  WARNING: Auto-approving commands without --allow-unsafe-exec.");
        eprintln!("[ai-coder-agent] ⚠️  This is risky as model-generated commands could be harmful.");
    }

    let commands = extract_commands(response);

    for code_block in commands {
        eprintln!("\n[ai-coder-agent] Found bash command(s):");
        eprintln!("{}", "=".repeat(60));
        eprintln!("{}", code_block);
        eprintln!("{}", "=".repeat(60));

        if !auto_approve {
            eprint!("\n[ai-coder-agent] Execute? (y/n): ");
            io::stderr().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if !input.trim().eq_ignore_ascii_case("y") {
                eprintln!("[ai-coder-agent] Skipped.");
                continue;
            }
        }

        // Execute the command
        execute_bash(&code_block)?;
    }

    Ok(())
}

fn extract_commands(response: &str) -> Vec<String> {
    let mut commands = Vec::new();
    let mut in_code_block = false;
    let mut code_block = String::new();
    let mut language = String::new();

    for line in response.lines() {
        // Detect code block start
        if line.trim().starts_with("```") {
            if in_code_block {
                // End of code block
                in_code_block = false;

                // Execute if it's a bash block
                let lang_token = language.split_whitespace().next().unwrap_or("");
                if lang_token.is_empty() || lang_token == "bash" || lang_token == "sh" {
                    commands.push(mem::take(&mut code_block));
                } else {
                    code_block.clear();
                }
                language.clear();
            } else {
                // Start of code block
                in_code_block = true;
                language = line.trim().strip_prefix("```").unwrap_or("").to_string();
            }
        } else if in_code_block {
            code_block.push_str(line);
            code_block.push('\n');
        }
    }

    if in_code_block {
        let lang_token = language.split_whitespace().next().unwrap_or("");
        if lang_token.is_empty() || lang_token == "bash" || lang_token == "sh" {
            commands.push(code_block);
        }
    }

    commands
}

/// Executes a string as a bash script.
pub fn execute_bash(script: &str) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("\n[ai-coder-agent] Executing...");
    let status = Command::new("bash").arg("-c").arg(script).status()?;

    if !status.success() {
        eprintln!(
            "[ai-coder-agent] ⚠️  Command failed with status: {}",
            status
        );
    } else {
        eprintln!("[ai-coder-agent] ✓ Command succeeded");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_commands_bash() {
        let response = "Here is a command:\n```bash\necho hello\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0], "echo hello\n");
    }

    #[test]
    fn test_extract_commands_sh() {
        let response = "Here is a command:\n```sh\necho hello\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0], "echo hello\n");
    }

    #[test]
    fn test_extract_commands_no_lang() {
        let response = "Here is a command:\n```\necho hello\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0], "echo hello\n");
    }

    #[test]
    fn test_extract_commands_other_lang() {
        let response = "Here is some rust code:\n```rust\nfn main() {}\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_extract_multiple_commands() {
        let response = "First:\n```bash\necho one\n```\nSecond:\n```sh\necho two\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0], "echo one\n");
        assert_eq!(commands[1], "echo two\n");
    }

    #[test]
    fn test_extract_commands_mixed_langs() {
        let response = "Rust:\n```rust\nprintln!(\"hi\");\n```\nBash:\n```bash\necho hi\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0], "echo hi\n");
    }

    #[test]
    fn test_extract_commands_empty_block() {
        let response = "Empty:\n```bash\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0], "");
    }

    #[test]
    fn test_extract_commands_with_extra_spaces() {
        let response = "  ```bash  \necho spaced\n  ```  ";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0], "echo spaced\n");
    }

    #[test]
    fn test_extract_commands_unclosed_block() {
        let response = "Run this:\n```bash\necho no-close";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0], "echo no-close\n");
    }

    #[test]
    fn test_extract_commands_precise_language_match() {
        let response = "```fish\necho should-not-run\n```";
        let commands = extract_commands(response);
        assert_eq!(commands.len(), 0);
    }
}
