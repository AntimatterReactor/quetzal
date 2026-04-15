use criterion::{black_box, criterion_group, criterion_main, Criterion};
use libquetzal::Lexer;

fn lexer_benchmark(c: &mut Criterion) {
    // Simple test case
    let source = "let x = 5 + 3 * 2";

    c.bench_function("lexer_simple", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            let _tokens = lexer.lexicalize();
        })
    });
    
    // You can add more complex test cases here
    let complex_source = "function add(a, b) {\n  return a + b;\n}\n\nlet result = add(5, 10);";
    
    c.bench_function("lexer_complex", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(complex_source));
            let _tokens = lexer.lexicalize();
        })
    });

    let mut group = c.benchmark_group("lexer_by_size");
    
    for size in [10, 100, 1000, 10000].iter() {
        // Generate a string of appropriate size
        let input = "let x = 42;".repeat(*size / 10);
        
        group.bench_with_input(format!("input_size_{}", size), &input, |b, input| {
            b.iter(|| {
                let mut lexer = Lexer::new(black_box(input));
                let _tokens = lexer.lexicalize();
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, lexer_benchmark);
criterion_main!(benches);
