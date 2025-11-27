use std::sync::LazyLock;
use tracing::warn;

pub static CACHE_MAX_SIZE: LazyLock<usize> = LazyLock::new(|| {
    static VAR: &str = "PIXELSHIFT_CACHE_MAX_SIZE";
    static DEFAULT: &str = "100";

    let value: usize = std::env::var(VAR.to_string())
        .unwrap_or_else(|err| {
            warn!("Using default value {} for {}: {}", DEFAULT, VAR, err);

            DEFAULT.to_string()
        })
        .parse::<usize>()
        .expect(format!("Error happened parsing value for {}", VAR.to_string()).as_str());

    value
});

pub static MAX_DOWNLOAD_SIZE_BYTES: LazyLock<u64> = LazyLock::new(|| {
    static VAR: &str = "PIXELSHIFT_MAX_DOWNLOAD_SIZE_BYTES";
    static DEFAULT: &str = "5000000"; // 5MB

    let value: u64 = std::env::var(VAR.to_string())
        .unwrap_or_else(|err| {
            warn!("Using default value {} for {}: {}", DEFAULT, VAR, err);

            DEFAULT.to_string()
        })
        .parse::<u64>()
        .expect(format!("Error happened parsing value for {}", VAR.to_string()).as_str());

    value
});

pub static HOST: LazyLock<String> = LazyLock::new(|| {
    static VAR: &str = "PIXELSHIFT_HOST";
    static DEFAULT: &str = "127.0.0.1";

    let value: String = std::env::var(VAR.to_string()).unwrap_or_else(|err| {
        warn!("Using default value {} for {}: {}", DEFAULT, VAR, err);

        DEFAULT.to_string()
    });

    value
});

pub static PORT: LazyLock<String> = LazyLock::new(|| {
    static VAR: &str = "PIXELSHIFT_PORT";
    static DEFAULT: &str = "3000";

    let value: String = std::env::var(VAR.to_string()).unwrap_or_else(|err| {
        warn!("Using default value {} for {}: {}", DEFAULT, VAR, err);

        DEFAULT.to_string()
    });

    value
});
