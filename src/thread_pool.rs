use num_cpus::get;

pub fn thread_pool_size() -> usize {
    (get() as f64 * 1.5) as usize
}
