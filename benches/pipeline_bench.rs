use agent_data_pipeline_rust::{transform, PipelineOptions, Record};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_transform(c: &mut Criterion) {
    let data: Vec<Record> = (0..100_000)
        .map(|i| Record {
            id: i,
            name: format!("user-{i}"),
            amount: (i % 100) as f64,
            category: if i % 2 == 0 { "retail" } else { "saas" }.to_string(),
        })
        .collect();

    let opts = PipelineOptions {
        min_amount: Some(50.0),
        uppercase_name: true,
        category_equals: Some("retail".into()),
    };

    c.bench_function("transform_100k", |b| {
        b.iter(|| {
            let out = transform(data.clone(), &opts);
            criterion::black_box(out);
        })
    });
}

criterion_group!(benches, bench_transform);
criterion_main!(benches);
