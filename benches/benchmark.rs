// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use bump2version::config::{BumpConfig, FileConfig, parse_config};
use bump2version::files::apply_file_change;
use bump2version::version::{bump_version, parse_version, serialize_version};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

static MINIMAL_CONFIG: &str = r#"
[bumpversion]
current_version = 1.2.3
commit = true
tag = false

[bumpversion:file:Cargo.toml]
search = version = "{current_version}"
replace = version = "{new_version}"
"#;

static FULL_CONFIG: &str = r#"
[bumpversion]
current_version = 1.2.3
commit = true
tag = true
parse = (?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(-(?P<stage>[^.]*?)\.(?P<devnum>\d+))?
serialize =
    {major}.{minor}.{patch}-{stage}.{devnum}
    {major}.{minor}.{patch}

[bumpversion:part:stage]
optional_value = stable
first_value = stable
values =
    alpha
    beta
    stable

[bumpversion:part:devnum]

[bumpversion:file:Cargo.toml]
search = version = "{current_version}"
replace = version = "{new_version}"

[bumpversion:file:README.md]
search = ## {current_version}
replace = ## {new_version}
"#;

fn make_large_file(lines: usize) -> String {
    let mut s = String::with_capacity(lines * 40);
    for i in 0..lines {
        if i == lines / 2 {
            s.push_str("version = \"1.2.3\"\n");
        } else {
            s.push_str(&format!("# line {}\n", i));
        }
    }
    s
}

fn bench_config_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_parse");

    group.bench_function("minimal", |b| {
        b.iter(|| parse_config(black_box(MINIMAL_CONFIG)).unwrap())
    });

    group.bench_function("full_with_parts", |b| {
        b.iter(|| parse_config(black_box(FULL_CONFIG)).unwrap())
    });

    group.finish();
}

fn bench_version_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("version_parse");
    let cfg = BumpConfig::default();

    for ver in &["1.0.0", "12.345.6789", "0.0.1"] {
        group.bench_with_input(BenchmarkId::new("parse", ver), ver, |b, v| {
            b.iter(|| parse_version(black_box(v), black_box(&cfg)).unwrap())
        });
    }

    group.finish();
}

fn bench_version_bump(c: &mut Criterion) {
    let mut group = c.benchmark_group("version_bump");
    let cfg = BumpConfig::default();
    let version = parse_version("1.2.3", &cfg).unwrap();

    for part in &["patch", "minor", "major"] {
        group.bench_with_input(BenchmarkId::new("bump", part), part, |b, p| {
            b.iter(|| {
                let bumped =
                    bump_version(black_box(&version), black_box(p), black_box(&cfg)).unwrap();
                serialize_version(black_box(&bumped), black_box(&cfg))
            })
        });
    }

    group.finish();
}

fn bench_file_replace(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_replace");
    let cfg = BumpConfig::default();
    let mut fc = FileConfig::new("bench.txt");
    fc.search = Some(r#"version = "{current_version}""#.to_string());
    fc.replace = Some(r#"version = "{new_version}""#.to_string());

    for size in &[100usize, 1_000, 10_000] {
        let content = make_large_file(*size);
        group.bench_with_input(BenchmarkId::new("lines", size), &content, |b, c_str| {
            b.iter(|| {
                apply_file_change(
                    black_box(c_str),
                    black_box(&fc),
                    black_box(&cfg),
                    black_box("1.2.3"),
                    black_box("1.2.4"),
                )
                .unwrap()
            })
        });
    }

    group.finish();
}

fn bench_multiline_replace(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiline_replace");
    let cfg = BumpConfig::default();
    let mut fc = FileConfig::new("CHANGELOG.md");
    fc.search = Some("## {current_version}\nsome context\nmore context".to_string());
    fc.replace = Some("## {new_version}\nsome context\nmore context".to_string());

    let content = "## 1.2.3\nsome context\nmore context\n\n## older\n".repeat(10);

    group.bench_function("multiline", |b| {
        b.iter(|| {
            apply_file_change(
                black_box(&content),
                black_box(&fc),
                black_box(&cfg),
                black_box("1.2.3"),
                black_box("1.2.4"),
            )
            .unwrap()
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_config_parse,
    bench_version_parse,
    bench_version_bump,
    bench_file_replace,
    bench_multiline_replace,
);
criterion_main!(benches);

// Copyright 2026 Mahmoud Harmouch.
//
// Licensed under the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
