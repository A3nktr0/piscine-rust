use chrono::Datelike;
use std::collections::HashMap;

pub struct CommitData {
    pub author: String,
    pub commits: u32,
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week = HashMap::new();
    for commit in data.members() {
        let date = commit["commit"]["author"]["date"].to_string();
        let parts: Vec<&str> = date.split('T').collect();
        let week = {
            let tmp_date = chrono::NaiveDate::parse_from_str(parts[0], "%Y-%m-%d")
                .unwrap()
                .iso_week();
            format!("{}-W{}", tmp_date.year(), tmp_date.week())
        };

        commits_per_week
            .entry(week)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    commits_per_week
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author = HashMap::new();
    for commit in data.members() {
        let author = commit["author"]["login"].to_string();
        commits_per_author
            .entry(author)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }
    commits_per_author
}
