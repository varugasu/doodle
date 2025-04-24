use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use toml::Table;

#[derive(Debug)]
struct CommandConfig {
    run: Vec<String>,
}

#[derive(Debug)]
struct Config {
    templates: HashMap<String, CommandConfig>,
}

impl Config {
    fn from_toml(table: Table) -> Result<Self, String> {
        let mut templates = HashMap::new();

        for (key, value) in table {
            let run = value
                .get("run")
                .and_then(|c| c.as_array())
                .ok_or_else(|| format!("Missing or invalid 'run' for key '{}'", key))?
                .iter()
                .map(|v| v.as_str().unwrap_or_default().to_string())
                .collect();

            templates.insert(key, CommandConfig { run });
        }

        Ok(Config { templates })
    }
}

fn read_config() -> Result<Config, String> {
    let home = std::env::var("HOME").map_err(|_| "Could not find HOME directory")?;
    let config_path = PathBuf::from(home).join("doodle.toml");

    let contents = fs::read_to_string(&config_path)
        .map_err(|e| format!("Error reading config file: {}", e))?;

    let table: Table = contents
        .parse()
        .map_err(|e| format!("Error parsing TOML: {}", e))?;

    Config::from_toml(table)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("\nUsage: doodle COMMAND");
        return;
    }

    match args[1].as_str() {
        "start" => {
            if args.len() == 2 {
                println!("Usage: doodle start TEMPLATE");
                return;
            }

            let template = args[2].clone();
            match read_config() {
                Ok(config) => {
                    let template = config.templates.get(&template).unwrap();
                    for command in template.run.clone() {
                        println!("Executing: {}", command);

                        let output = Command::new("sh").arg("-c").arg(&command).output();

                        match output {
                            Ok(output) => {
                                if output.status.success() {
                                    println!("✓ Command succeeded");
                                    if !output.stdout.is_empty() {
                                        println!("{}", String::from_utf8_lossy(&output.stdout));
                                    }
                                } else {
                                    eprintln!("✗ Command failed");
                                    if !output.stderr.is_empty() {
                                        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                                    }
                                    break; // Stop executing further commands
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to execute command: {}", e);
                                break; // Stop executing further commands
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Failed to load config: {}", e),
            }
        }
        _ => {
            println!("Unknown command");
        }
    }
}
