use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Record {
    pub id: u64,
    pub name: String,
    pub amount: f64,
    pub category: String,
}

#[derive(Debug, Clone, Default)]
pub struct PipelineOptions {
    pub min_amount: Option<f64>,
    pub uppercase_name: bool,
    pub category_equals: Option<String>,
}

pub fn transform(records: Vec<Record>, opts: &PipelineOptions) -> Vec<Record> {
    let mut out: Vec<Record> = records
        .into_par_iter()
        .filter(|r| opts.min_amount.map(|m| r.amount >= m).unwrap_or(true))
        .filter(|r| {
            opts.category_equals
                .as_ref()
                .map(|c| &r.category == c)
                .unwrap_or(true)
        })
        .map(|mut r| {
            if opts.uppercase_name {
                r.name = r.name.to_uppercase();
            }
            r
        })
        .collect();

    out.sort_by_key(|r| r.id);
    out
}

pub fn ingest_csv(path: &str) -> anyhow::Result<Vec<Record>> {
    let mut reader = csv::ReaderBuilder::new().from_path(path)?;
    let mut out = Vec::new();
    for r in reader.deserialize() {
        out.push(r?);
    }
    Ok(out)
}

pub fn export_json(path: &str, records: &[Record]) -> anyhow::Result<()> {
    let file = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(file, records)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_filters_and_uppercases() {
        let input = vec![
            Record {
                id: 1,
                name: "alice".into(),
                amount: 10.0,
                category: "retail".into(),
            },
            Record {
                id: 2,
                name: "bob".into(),
                amount: 2.0,
                category: "saas".into(),
            },
        ];
        let opts = PipelineOptions {
            min_amount: Some(5.0),
            uppercase_name: true,
            category_equals: Some("retail".into()),
        };

        let out = transform(input, &opts);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "ALICE");
        assert_eq!(out[0].id, 1);
    }

    #[test]
    fn transform_returns_stable_order_by_id() {
        let input = vec![
            Record {
                id: 9,
                name: "zeta".into(),
                amount: 10.0,
                category: "retail".into(),
            },
            Record {
                id: 3,
                name: "alpha".into(),
                amount: 11.0,
                category: "retail".into(),
            },
            Record {
                id: 5,
                name: "beta".into(),
                amount: 12.0,
                category: "retail".into(),
            },
        ];

        let out = transform(input, &PipelineOptions::default());
        let ids: Vec<u64> = out.into_iter().map(|r| r.id).collect();
        assert_eq!(ids, vec![3, 5, 9]);
    }
}
