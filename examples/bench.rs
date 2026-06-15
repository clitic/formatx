use std::{fmt::Write, fs, time::Instant};

const ITERATIONS: u32 = 1_000_000;

fn generate_svg(results: &[BenchResult]) -> String {
    let max_ns = results
        .iter()
        .flat_map(|r| [r.format_ns, r.formatx_ns])
        .fold(0.0_f64, f64::max);

    let chart_h = 240;
    let top = 90;
    let bottom = top + chart_h;
    let group_w: usize = 140;
    let bar_w: usize = 45;
    let gap: usize = 8;
    let left: usize = 70;
    let total_w = left + group_w * results.len() + 20;
    let total_h = bottom + 60;

    let mut svg = String::new();
    let _ = writeln!(
        svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {total_w} {total_h}\" font-family=\"'Segoe UI', system-ui, sans-serif\">"
    );
    let _ = writeln!(
        svg,
        "<rect width=\"{total_w}\" height=\"{total_h}\" fill=\"#1e293b\"/>"
    );

    // Title
    let cx = total_w / 2;
    let _ = writeln!(
        svg,
        "<text x=\"{cx}\" y=\"28\" text-anchor=\"middle\" fill=\"#f1f5f9\" font-size=\"16\" font-weight=\"700\">formatx! vs format!</text>"
    );
    let _ = writeln!(
        svg,
        "<text x=\"{cx}\" y=\"44\" text-anchor=\"middle\" fill=\"#94a3b8\" font-size=\"11\">{ITERATIONS} iterations \u{00b7} release mode \u{00b7} lower is better</text>"
    );

    // Legend
    let lx = cx - 70;
    let _ = writeln!(
        svg,
        "<rect x=\"{lx}\" y=\"52\" width=\"12\" height=\"12\" fill=\"#818cf8\"/><text x=\"{}\" y=\"63\" fill=\"#c7d2fe\" font-size=\"11\">format!</text>",
        lx + 17
    );
    let rx = cx + 20;
    let _ = writeln!(
        svg,
        "<rect x=\"{rx}\" y=\"52\" width=\"12\" height=\"12\" fill=\"#fb923c\"/><text x=\"{}\" y=\"63\" fill=\"#fed7aa\" font-size=\"11\">formatx!</text>",
        rx + 17
    );

    // Y-axis label
    let mid_y = top + chart_h / 2;
    let _ = writeln!(
        svg,
        "<text x=\"14\" y=\"{mid_y}\" text-anchor=\"middle\" fill=\"#64748b\" font-size=\"10\" transform=\"rotate(-90, 14, {mid_y})\">(ns/iter)</text>"
    );

    // Y-axis gridlines
    for i in 0..=4 {
        let scale = max_ns * 1.15;
        let ns = scale * i as f64 / 4.0;
        let y = bottom - (chart_h as f64 * i as f64 / 4.0) as usize;
        let _ = writeln!(
            svg,
            "<line x1=\"{left}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"#334155\" stroke-width=\"1\"/>",
            left + group_w * results.len()
        );
        let _ = writeln!(
            svg,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\" fill=\"#64748b\" font-size=\"10\">{:.0}</text>",
            left - 6,
            y + 4,
            ns
        );
    }

    // Bars
    for (i, r) in results.iter().enumerate() {
        let gx = left + i * group_w + (group_w - bar_w * 2 - gap) / 2;

        let scale = max_ns * 1.15;
        let h1 = (r.format_ns / scale * chart_h as f64).max(2.0) as usize;
        let h2 = (r.formatx_ns / scale * chart_h as f64).max(2.0) as usize;

        let x1 = gx;
        let x2 = gx + bar_w + gap;
        let y1 = bottom - h1;
        let y2 = bottom - h2;

        // format! bar + label
        let _ = writeln!(
            svg,
            "<rect x=\"{x1}\" y=\"{y1}\" width=\"{bar_w}\" height=\"{h1}\" fill=\"#818cf8\"/>"
        );
        let _ = writeln!(
            svg,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#c7d2fe\" font-size=\"10\">{:.0} ns</text>",
            x1 + bar_w / 2,
            y1 - 6,
            r.format_ns
        );

        // formatx! bar + label
        let _ = writeln!(
            svg,
            "<rect x=\"{x2}\" y=\"{y2}\" width=\"{bar_w}\" height=\"{h2}\" fill=\"#fb923c\"/>"
        );
        let _ = writeln!(
            svg,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" fill=\"#fed7aa\" font-size=\"10\">{:.0} ns</text>",
            x2 + bar_w / 2,
            y2 - 6,
            r.formatx_ns
        );

        // X-axis label
        let center_x = gx + (2 * bar_w + gap) / 2;
        let _ = writeln!(
            svg,
            "<text x=\"{center_x}\" y=\"{}\" text-anchor=\"middle\" fill=\"#e2e8f0\" font-size=\"11\">{}</text>",
            bottom + 16,
            r.label
        );
    }

    // Footer
    let _ = writeln!(
        svg,
        "<text x=\"{cx}\" y=\"{}\" text-anchor=\"middle\" fill=\"#64748b\" font-size=\"10\" font-style=\"italic\">Expected overhead - formatx! parses and resolves format strings at runtime.</text>",
        total_h - 10
    );

    let _ = writeln!(svg, "</svg>");
    svg
}

struct BenchResult {
    label: &'static str,
    format_ns: f64,
    formatx_ns: f64,
}

fn bench<F: Fn() -> String>(f: F) -> f64 {
    for _ in 0..1000 {
        std::hint::black_box(f());
    }
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        std::hint::black_box(f());
    }
    start.elapsed().as_nanos() as f64 / ITERATIONS as f64
}

fn main() {
    let results = vec![
        BenchResult {
            label: "{} + {} = {}",
            format_ns: bench(|| format!("{} + {} = {}", 1, 2, 3)),
            formatx_ns: bench(|| formatx::formatx!("{} + {} = {}", 1, 2, 3).unwrap()),
        },
        BenchResult {
            label: "{name} scored {score}%",
            format_ns: bench(|| format!("{name} scored {score}%", name = "Alice", score = 95)),
            formatx_ns: bench(|| {
                formatx::formatx!("{name} scored {score}%", name = "Alice", score = 95).unwrap()
            }),
        },
        BenchResult {
            label: "{:+08.2}",
            format_ns: bench(|| format!("{:+08.2}", 3.14159)),
            formatx_ns: bench(|| formatx::formatx!("{:+08.2}", 3.14159).unwrap()),
        },
        BenchResult {
            label: "{:-^20}",
            format_ns: bench(|| format!("{:-^20}", "center")),
            formatx_ns: bench(|| formatx::formatx!("{:-^20}", "center").unwrap()),
        },
    ];

    let svg = generate_svg(&results);
    fs::create_dir_all("images").ok();
    fs::write("images/benchmark.svg", &svg).unwrap();
}
