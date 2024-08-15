use peak_alloc::PeakAlloc;
use test_context::AsyncTestContext;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

pub struct MangoContext {
    pub mango: String,
    pub mem_limit_mb: f32,
}

impl MangoContext {
    async fn new(mango: String) -> Self {
        println!("mango {}", mango);
        Self {
            mango,
            mem_limit_mb: 1.0,
        }
    }
}

impl AsyncTestContext for MangoContext {
    async fn setup() -> MangoContext {
        let mango = String::from("mango");
        MangoContext::new(mango).await
    }

    async fn teardown(self) {
        println!("tear-ing down....");
        let peak_mem = PEAK_ALLOC.peak_usage_as_mb();
        if peak_mem > self.mem_limit_mb {
            println!("Too much RAM used: {peak_mem} MB");
        }
        PEAK_ALLOC.reset_peak_usage();
    }
}

