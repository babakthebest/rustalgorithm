use std::time::Instant;
use sysinfo::{ System,Pid,ProcessesToUpdate};
use std::process;
use std::fmt::Debug;

// Function to measure performance
pub fn measure_performance<F, T>(func: F, func_name: &str) -> T
where
    F: FnOnce() -> T,
     T: Debug,  // Ensure T implements Debug
{
    // Measure memory before execution
    let mut system = System::new_all();
    let pid = Pid::from(process::id() as usize) ;
    system.refresh_processes(ProcessesToUpdate::All);
    let memory_before = system.process(pid).map_or(0, |p| p.memory());

    // Measure wall-clock time before execution
    let wall_clock_start = Instant::now();

    // Execute the function
    let result = func();

    // Measure wall-clock time after execution
    let wall_clock_end = Instant::now();

    // Measure memory after execution
    system.refresh_processes(ProcessesToUpdate::All);
    let memory_after = system.process(pid).map_or(0, |p| p.memory());

    // Report the results
    println!("{} results: {:?}", func_name, result);
    println!(
        "{} memory usage: {:.5?} KB",
        func_name,
        memory_after as i64 - memory_before as i64
    );
    println!(
        "{} wall-clock time: {:.5?} ms",
        func_name,
        wall_clock_end.duration_since(wall_clock_start).as_secs_f64() * 1000.0
    );

    result
}
