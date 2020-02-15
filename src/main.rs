use std::fs::read_to_string;
use std::path::PathBuf;
use std::io::Result;

const CAP: &str = "capacity";
const STATUS: &str = "status";
const ROOT: &str = "/sys/class/power_supply/";
const BAT0: &str = "BAT0";

fn main() -> Result<()> {
    let root = PathBuf::from(ROOT).join(BAT0);
    if !root.exists() {
        print!("");
        return Ok(());
    }

    let capacity_file = root.join(CAP); 
    let status_file = root.join(STATUS); 

    let capacity = read_to_string(capacity_file)?;

    let status = read_to_string(status_file)?;

    let status = match &status.trim().to_lowercase() as &str{
        "discharging" => "",
        "charging" => "",
        "full" => "",
        _ => "?"
    };

    let output = format!("#[fg=colour66,bg=colour234]#[bg=colour66,fg=colour234] {status}{capacity}% #[fg=colour234,bg=colour66]", capacity=capacity.trim(), status=status);
    // let output = format!("#[fg=colour196,bg=colour234]#[bg=colour196,fg=colour234] {status}{capacity}% #[fg=colour234,bg=colour196]", capacity=capacity.trim(), status=status);
    print!("{}", output);

    Ok(())
}
