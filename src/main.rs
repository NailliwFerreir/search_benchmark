mod types;
mod search;
mod csv_export;

use rand::Rng;
use types::{SearchResult, BenchmarkResults};
use search::{sequential_search, binary_search};
use csv_export::{export_to_csv, generate_analysis_summary};

fn generate_sorted_array(size: usize) -> Vec<i32> {
    (1..=size as i32).collect()
}

fn run_benchmark(size: usize) -> BenchmarkResults {
    let arr = generate_sorted_array(size);
    let mut rng = rand::thread_rng();
    
    let random_index = rng.gen_range(0..size);
    let avg_case_target = arr[random_index];
    let worst_case_target = (size as i32) + 1000;
    
    println!("Testando {} elementos...", size);
    
    let iterations = 1000;
    
    // Busca Sequencial - Caso Médio
    let mut seq_avg_time = 0u128;
    let mut seq_avg_positions = 0usize;
    for _ in 0..iterations {
        let result = sequential_search(&arr, avg_case_target);
        seq_avg_time += result.execution_time_ns;
        seq_avg_positions += result.positions_visited;
    }
    let sequential_avg_case = SearchResult {
        positions_visited: seq_avg_positions / iterations,
        execution_time_ns: seq_avg_time / iterations as u128,
    };
    
    // Busca Sequencial - Pior Caso
    let mut seq_worst_time = 0u128;
    let mut seq_worst_positions = 0usize;
    for _ in 0..iterations {
        let result = sequential_search(&arr, worst_case_target);
        seq_worst_time += result.execution_time_ns;
        seq_worst_positions += result.positions_visited;
    }
    let sequential_worst_case = SearchResult {
        positions_visited: seq_worst_positions / iterations,
        execution_time_ns: seq_worst_time / iterations as u128,
    };
    
    // Busca Binária - Caso Médio
    let mut bin_avg_time = 0u128;
    let mut bin_avg_positions = 0usize;
    for _ in 0..iterations {
        let result = binary_search(&arr, avg_case_target);
        bin_avg_time += result.execution_time_ns;
        bin_avg_positions += result.positions_visited;
    }
    let binary_avg_case = SearchResult {
        positions_visited: bin_avg_positions / iterations,
        execution_time_ns: bin_avg_time / iterations as u128,
    };
    
    // Busca Binária - Pior Caso
    let mut bin_worst_time = 0u128;
    let mut bin_worst_positions = 0usize;
    for _ in 0..iterations {
        let result = binary_search(&arr, worst_case_target);
        bin_worst_time += result.execution_time_ns;
        bin_worst_positions += result.positions_visited;
    }
    let binary_worst_case = SearchResult {
        positions_visited: bin_worst_positions / iterations,
        execution_time_ns: bin_worst_time / iterations as u128,
    };
    
    BenchmarkResults {
        size,
        sequential_avg_case,
        sequential_worst_case,
        binary_avg_case,
        binary_worst_case,
    }
}

fn print_results(results: &BenchmarkResults) {
    println!("\n=== RESULTADOS PARA {} ELEMENTOS ===", results.size);
    
    println!("\n{:<20} {:>12} {:>12} {:>12} {:>12}", 
             "ALGORITMO", "POS MÉDIA", "POS PIOR", "TEMPO MÉD", "TEMPO PIOR");
    println!("{:-<72}", "");
    
    println!("{:<20} {:>12} {:>12} {:>12} {:>12}",
             "Busca Sequencial",
             results.sequential_avg_case.positions_visited,
             results.sequential_worst_case.positions_visited,
             format!("{}ns", results.sequential_avg_case.execution_time_ns),
             format!("{}ns", results.sequential_worst_case.execution_time_ns));
    
    println!("{:<20} {:>12} {:>12} {:>12} {:>12}",
             "Busca Binária",
             results.binary_avg_case.positions_visited,
             results.binary_worst_case.positions_visited,
             format!("{}ns", results.binary_avg_case.execution_time_ns),
             format!("{}ns", results.binary_worst_case.execution_time_ns));
    
    let speedup_avg = results.sequential_avg_case.execution_time_ns as f64 / 
                      results.binary_avg_case.execution_time_ns as f64;
    let speedup_worst = results.sequential_worst_case.execution_time_ns as f64 / 
                        results.binary_worst_case.execution_time_ns as f64;
    
    println!("\nSpeedup: {:.1}x (médio) | {:.1}x (pior)", speedup_avg, speedup_worst);
}

fn main() {
    println!("🦀 BENCHMARK: Busca Sequencial vs Busca Binária");
    println!("Testando com 1.000 iterações por cenário\n");
    
    let sizes = vec![1_000, 10_000, 100_000];
    let mut all_results = Vec::new();
    
    for size in sizes {
        let results = run_benchmark(size);
        print_results(&results);
        all_results.push(results);
        println!();
    }
    
    // Exportar dados para CSV
    if let Err(e) = export_to_csv(&all_results) {
        println!("❌ Erro ao exportar CSV: {}", e);
    }
    
    // Análise para criação de gráficos
    generate_analysis_summary(&all_results);
    
    println!("\n=== RESUMO FINAL ===");
    println!("Complexidade: Sequencial O(n) | Binária O(log n)");
    for result in &all_results {
        let ratio = result.sequential_worst_case.positions_visited as f64 / 
                   result.binary_worst_case.positions_visited as f64;
        println!("{:>6} elementos: Binária {:.1}x mais eficiente", result.size, ratio);
    }
    
    println!("\n📋 ARQUIVO GERADO:");
    println!("   • benchmark_results.csv (com todas as métricas)");
    println!("\n💡 Use esse CSV para criar gráficos no Excel, Python, R, etc.");
}