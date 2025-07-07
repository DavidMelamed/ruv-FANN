use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait MarketingAgent {
    async fn plan(&self, input: &str) -> anyhow::Result<String>;
    async fn act(&self, plan: &str) -> anyhow::Result<String>;
}

pub mod agents {
    use super::*;

    macro_rules! stub_agent {
        ($name:ident) => {
            pub struct $name;
            #[async_trait]
            impl MarketingAgent for $name {
                async fn plan(&self, _input: &str) -> anyhow::Result<String> {
                    Ok("TODO: plan".to_string())
                }
                async fn act(&self, _plan: &str) -> anyhow::Result<String> {
                    Ok("TODO: act".to_string())
                }
            }
        };
    }

    stub_agent!(SeoCrawler);
    stub_agent!(TrendHijacker);
    stub_agent!(UgcForge);
    stub_agent!(AdCreativePolisher);
    stub_agent!(AnalyticsTruthSeeker);
    stub_agent!(BacklinkDiplomat);
}
