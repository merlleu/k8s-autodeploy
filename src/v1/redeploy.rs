use std::collections::BTreeMap;

use actix_web::{
    post,
    web::{Data, JsonBody, Json},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Deserialize, Serialize)]
pub struct PostRedeployBody {
    pub image: String,
}

#[post("/redeploy")]
pub async fn post_redeploy(
    req: HttpRequest,
    body: Json<PostRedeployBody>,
    ctx: Data<Context>,
) -> HttpResponse {
    if !check_auth(&req, &ctx) {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    let body = body.into_inner();

    let deployments = list_deployments_by_image(&ctx, &body.image).await;

    for deployment in &deployments {
        redeploy_deployment(&ctx, deployment).await;
    }

    HttpResponse::Ok().json(&deployments)
}

fn check_auth(req: &HttpRequest, ctx: &Context) -> bool {
    if req.headers().get("Authorization").is_none() {
        return false;
    }

    let auth = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    ctx.config.webhook_secret == auth
}

async fn list_deployments(ctx: &Context) -> Vec<RancherDeployment> {
    let j: RancherResult<Vec<RancherDeployment>> = ctx
        .client
        .get(&format!(
            "{}/k8s/clusters/{}/v1/apps.deployments",
            ctx.config.rancher_url, ctx.config.rancher_cluster_id
        ))
        .bearer_auth(&ctx.config.rancher_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    j.data
}

async fn list_deployments_by_image(ctx: &Context, image: &str) -> Vec<String> {
    let deployments = list_deployments(ctx).await;
    let mut result = Vec::new();

    for deployment in deployments {
        for container in deployment.spec.template.spec.containers {
            if container.image == image {
                result.push(deployment.id.clone());
            }
        }
    }

    result
}

async fn get_deployment_by_id(ctx: &Context, id: &str) -> RancherDeployment {
    let j: RancherDeployment = ctx
        .client
        .get(&format!(
            "{}/k8s/clusters/{}/v1/apps.deployments/{}",
            ctx.config.rancher_url, ctx.config.rancher_cluster_id, id
        ))
        .bearer_auth(&ctx.config.rancher_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    j
}

async fn redeploy_deployment(ctx: &Context, id: &str) {
    let mut old = get_deployment_by_id(ctx, id).await;

    old.spec.template.metadata.annotations.insert(
        "eu.merll.autodeploy/timestamp".to_string(),
        chrono::Utc::now().to_rfc3339(),
    );

    let j = ctx
        .client
        .put(&format!(
            "{}/k8s/clusters/{}/v1/apps.deployments/{}",
            ctx.config.rancher_url, ctx.config.rancher_cluster_id, id
        ))
        .bearer_auth(&ctx.config.rancher_token)
        .json(&old)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    // println!("{:?}", j);
}

#[derive(Deserialize, Serialize)]
struct RancherResult<T> {
    pub data: T,
}

#[derive(Deserialize, Serialize)]
struct RancherDeployment {
    pub id: String,
    pub spec: RancherDeploymentSpec,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
struct RancherDeploymentSpec {
    pub template: RancherDeploymentTemplate,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
struct RancherDeploymentTemplate {
    pub spec: RancherDeploymentTemplateSpec,
    pub metadata: RancherDeploymentTemplateMetadata,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
struct RancherDeploymentTemplateSpec {
    pub containers: Vec<RancherDeploymentTemplateSpecContainer>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
struct RancherDeploymentTemplateSpecContainer {
    pub image: String,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

#[derive(Deserialize, Serialize)]
struct RancherDeploymentTemplateMetadata {
    #[serde(default)]
    pub annotations: BTreeMap<String, String>,
    #[serde(flatten)]
    pub other: BTreeMap<String, serde_json::Value>,
}

