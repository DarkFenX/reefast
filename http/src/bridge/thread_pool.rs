pub(crate) struct HThreadPool {
    pub(in crate::bridge) standard: tokio_rayon::rayon::ThreadPool,
    pub(in crate::bridge) heavy: tokio_rayon::rayon::ThreadPool,
}
impl HThreadPool {
    pub(crate) fn new(std_threads: usize, heavy_threads: usize) -> Self {
        Self {
            standard: tokio_rayon::rayon::ThreadPoolBuilder::new()
                .num_threads(std_threads)
                .build()
                .unwrap(),
            heavy: tokio_rayon::rayon::ThreadPoolBuilder::new()
                .num_threads(heavy_threads)
                .build()
                .unwrap(),
        }
    }
}
