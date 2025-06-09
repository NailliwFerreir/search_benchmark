use std::fs::File;
use std::io::Write;
use crate::types::BenchmarkResults;

pub fn export_to_csv(results: &[BenchmarkResults]) -> Result<(), Box<dyn std::error::Error>> {
    // CSV único com todos os dados organizados
    let mut file = File::create("benchmark_results.csv")?;
    
    // Cabeçalho com todas as métricas
    writeln!(file, "Size,Seq_Avg_Time_ns,Seq_Worst_Time_ns,Bin_Avg_Time_ns,Bin_Worst_Time_ns,Seq_Avg_Positions,Seq_Worst_Positions,Bin_Avg_Positions,Bin_Worst_Positions,Time_Speedup_Avg,Time_Speedup_Worst,Position_Efficiency_Avg,Position_Efficiency_Worst,Theoretical_Log2")?;
    
    for result in results {
        let size = result.size;
        
        // Cálculos de speedup e eficiência
        let time_speedup_avg = result.sequential_avg_case.execution_time_ns as f64 / 
                              result.binary_avg_case.execution_time_ns as f64;
        let time_speedup_worst = result.sequential_worst_case.execution_time_ns as f64 / 
                                result.binary_worst_case.execution_time_ns as f64;
        
        let pos_efficiency_avg = result.sequential_avg_case.positions_visited as f64 / 
                                result.binary_avg_case.positions_visited as f64;
        let pos_efficiency_worst = result.sequential_worst_case.positions_visited as f64 / 
                                  result.binary_worst_case.positions_visited as f64;
        
        let theoretical_log2 = (size as f64).log2();
        
        // Linha com todos os dados
        writeln!(file, "{},{},{},{},{},{},{},{},{},{:.2},{:.2},{:.2},{:.2},{:.2}",
                size,
                result.sequential_avg_case.execution_time_ns,
                result.sequential_worst_case.execution_time_ns,
                result.binary_avg_case.execution_time_ns,
                result.binary_worst_case.execution_time_ns,
                result.sequential_avg_case.positions_visited,
                result.sequential_worst_case.positions_visited,
                result.binary_avg_case.positions_visited,
                result.binary_worst_case.positions_visited,
                time_speedup_avg,
                time_speedup_worst,
                pos_efficiency_avg,
                pos_efficiency_worst,
                theoretical_log2)?;
    }
    
    println!("✅ Dados exportados para: benchmark_results.csv");
    println!("📊 Arquivo único com todas as métricas organizadas");
    
    Ok(())
}

pub fn generate_analysis_summary(results: &[BenchmarkResults]) {
    println!("\n=== ANÁLISE PARA GRÁFICOS ===");
    
    println!("\n📊 COMPLEXIDADE OBSERVADA:");
    for result in results {
        let theoretical_log = (result.size as f64).log2();
        let observed_avg = result.binary_avg_case.positions_visited as f64;
        let observed_worst = result.binary_worst_case.positions_visited as f64;
        
        println!("Tamanho {}: Log₂({}) = {:.1} | Observado: {:.1} (avg), {:.1} (worst)", 
                result.size, result.size, theoretical_log, observed_avg, observed_worst);
    }
    
    println!("\n📈 CRESCIMENTO RELATIVO:");
    if results.len() >= 2 {
        for i in 1..results.len() {
            let prev = &results[i-1];
            let curr = &results[i];
            
            let size_growth = curr.size as f64 / prev.size as f64;
            let seq_time_growth = curr.sequential_worst_case.execution_time_ns as f64 / 
                                 prev.sequential_worst_case.execution_time_ns as f64;
            let bin_time_growth = curr.binary_worst_case.execution_time_ns as f64 / 
                                 prev.binary_worst_case.execution_time_ns as f64;
            let seq_pos_growth = curr.sequential_worst_case.positions_visited as f64 / 
                                prev.sequential_worst_case.positions_visited as f64;
            let bin_pos_growth = curr.binary_worst_case.positions_visited as f64 / 
                                prev.binary_worst_case.positions_visited as f64;
            
            println!("{}x → {}x elementos:", prev.size, curr.size);
            println!("  Crescimento: {:.1}x | Seq tempo: {:.1}x | Bin tempo: {:.1}x", 
                    size_growth, seq_time_growth, bin_time_growth);
            println!("  Seq posições: {:.1}x | Bin posições: {:.1}x", 
                    seq_pos_growth, bin_pos_growth);
        }
    }
}