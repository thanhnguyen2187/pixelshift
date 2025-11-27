use std::sync::LazyLock;
use tracing::warn;

pub static CACHE_ITEM_MIN_SECONDS: LazyLock<i64> = LazyLock::new(|| {
    static VAR: &str = "PIXELSHIFT_CACHE_ITEM_MIN_SECONDS";
    static DEFAULT: &str = "60";

    let value: i64 = std::env::var(VAR.to_string())
        .unwrap_or_else(|err| {
            warn!("Using default value {} for {}: {}", DEFAULT, VAR, err);

            DEFAULT.to_string()
        })
        .parse::<i64>()
        .expect(format!("Error happened parsing value for {}", VAR.to_string()).as_str());

    value
});

pub static CACHE_TOTAL_MAX_BYTES: LazyLock<i64> = LazyLock::new(|| {
    static VAR: &str = "PIXELSHIFT_CACHE_TOTAL_MAX_BYTES";
    static DEFAULT: &str = "500000000"; // 500MB

    let value: i64 = std::env::var(VAR.to_string())
        .unwrap_or_else(|err| {
            warn!("Using default value {} for {}: {}", DEFAULT, VAR, err);

            DEFAULT.to_string()
        })
        .parse::<i64>()
        .expect(format!("Error happened parsing value for {}", VAR.to_string()).as_str());

    value
});
