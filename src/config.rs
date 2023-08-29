pub struct AppConfig {
    pub rancher_url: String,
    pub rancher_token: String,
    pub rancher_cluster_id: String,

    pub webhook_secret: String,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            rancher_url: std::env::var("RANCHER_URL").expect("RANCHER_URL is not set."),
            rancher_token: std::env::var("RANCHER_TOKEN").expect("RANCHER_TOKEN is not set."),
            rancher_cluster_id: std::env::var("RANCHER_CLUSTER_ID").expect("RANCHER_CLUSTER_ID is not set."),

            webhook_secret: std::env::var("WEBHOOK_SECRET").expect("WEBHOOK_SECRET is not set."),
        }
    }
}