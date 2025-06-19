use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufWriter, Write, BufRead};
use chrono::Local;

pub struct Expense {
    pub date: String,
    pub amount: f64,
    pub category: String,
    pub desc: String,
}

pub fn add_expense(amount: f64, category: &str, desc: &str) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("expenses.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "{},{},{},{}",
        Local::now().format("%Y-%m-%d"),
        amount,
        category,
        desc.replace(',', " ")
    )?;
    Ok(())
}

pub fn list_expenses() -> std::io::Result<Vec<Expense>> {
    let file = File::open("expenses.csv")?;
    let reader = BufReader::new(file);
    let mut res = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(4, ',').collect();
        if parts.len() == 4 {
            res.push(Expense {
                date: parts[0].to_string(),
                amount: parts[1].parse().unwrap_or(0.0),
                category: parts[2].to_string(),
                desc: parts[3].to_string(),
            });
        }
    }
    Ok(res)
}