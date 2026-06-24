use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <contract.sol>");
        return;
    }

    let filename = &args[1];

    let contract = std::fs::read_to_string(filename)
        .expect("Could not read contract file");

    let mut functions = vec![];
    let mut state_vars = vec![];

    for line in contract.lines() {
        let trimmed = line.trim();

        if trimmed.contains("function") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            if parts.len() > 1 {
                let name = parts[1]
                    .split('(')
                    .next()
                    .unwrap()
                    .to_string();

                functions.push(name);
            }
        }

        if trimmed.contains("public") || trimmed.contains("mapping") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();

            if let Some(last) = parts.last() {
                let var = last.replace(";", "");

                if !var.contains(")") {
                    state_vars.push(var);
                }
            }
        }
    }

    println!("=== AUDIT REPORT ===\n");

    println!("Functions Found:");
    for f in &functions {
        println!("- {}", f);
    }

    println!("\nState Variables:");
    for v in &state_vars {
        println!("- {}", v);
    }

    println!("\nFindings:");

    for f in &functions {
        if f.contains("withdraw") {
            println!("[HIGH] Withdrawal function detected. Review access control.");
        }

        if f.contains("transfer") {
            println!("[HIGH] Asset transfer function detected.");
        }
    }

    for v in &state_vars {
        if v == "owner" {
            println!("[MEDIUM] Centralized ownership detected.");
        }

        if v == "balances" {
            println!("[MEDIUM] Stores user funds. Review reentrancy protections.");
        }
    }

    println!("\n=== END REPORT ===");
}