use std::collections::HashMap;
use std::sync::Arc;

use axum::{extract::{Path, State}, routing::{get, post, put, delete}, Json, Router, Server};
use axum::response::IntoResponse;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tonic::{transport::Server as GrpcServer, Request, Response, Status};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/marketing.rs"));
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Campaign {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Creative {
    pub id: String,
    pub campaign_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Metric {
    pub id: String,
    pub creative_id: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Playbook {
    pub id: String,
    pub name: String,
}

impl From<proto::Campaign> for Campaign {
    fn from(p: proto::Campaign) -> Self {
        Self { id: p.id, name: p.name }
    }
}
impl From<Campaign> for proto::Campaign {
    fn from(c: Campaign) -> Self {
        Self { id: c.id, name: c.name }
    }
}
impl From<proto::Creative> for Creative {
    fn from(p: proto::Creative) -> Self {
        Self { id: p.id, campaign_id: p.campaign_id }
    }
}
impl From<Creative> for proto::Creative {
    fn from(c: Creative) -> Self {
        Self { id: c.id, campaign_id: c.campaign_id }
    }
}
impl From<proto::Metric> for Metric {
    fn from(p: proto::Metric) -> Self {
        Self { id: p.id, creative_id: p.creative_id, value: p.value }
    }
}
impl From<Metric> for proto::Metric {
    fn from(m: Metric) -> Self {
        Self { id: m.id, creative_id: m.creative_id, value: m.value }
    }
}
impl From<proto::Playbook> for Playbook {
    fn from(p: proto::Playbook) -> Self {
        Self { id: p.id, name: p.name }
    }
}
impl From<Playbook> for proto::Playbook {
    fn from(p: Playbook) -> Self {
        Self { id: p.id, name: p.name }
    }
}

#[derive(Default)]
struct Store {
    campaigns: RwLock<HashMap<String, Campaign>>,
    creatives: RwLock<HashMap<String, Creative>>,
    metrics: RwLock<HashMap<String, Metric>>,
    playbooks: RwLock<HashMap<String, Playbook>>,
}

#[utoipa::path(post, path = "/campaigns", request_body = Campaign, responses((status = 200, body = Campaign)))]
async fn create_campaign(State(store): State<Arc<Store>>, Json(c): Json<Campaign>) -> impl axum::response::IntoResponse {
    store.campaigns.write().insert(c.id.clone(), c.clone());
    Json(c)
}

#[utoipa::path(get, path = "/campaigns/{id}", responses((status = 200, body = Campaign)))]
async fn get_campaign(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    if let Some(c) = store.campaigns.read().get(&id).cloned() {
        Json(c).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(put, path = "/campaigns/{id}", request_body = Campaign, responses((status = 200, body = Campaign)))]
async fn update_campaign(State(store): State<Arc<Store>>, Path(id): Path<String>, Json(c): Json<Campaign>) -> impl axum::response::IntoResponse {
    let mut map = store.campaigns.write();
    if map.contains_key(&id) {
        map.insert(id.clone(), c.clone());
        Json(c).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(delete, path = "/campaigns/{id}", responses((status = 204)))]
async fn delete_campaign(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    store.campaigns.write().remove(&id);
    axum::http::StatusCode::NO_CONTENT
}

#[utoipa::path(post, path = "/creatives", request_body = Creative, responses((status = 200, body = Creative)))]
async fn create_creative(State(store): State<Arc<Store>>, Json(c): Json<Creative>) -> impl axum::response::IntoResponse {
    store.creatives.write().insert(c.id.clone(), c.clone());
    Json(c)
}

#[utoipa::path(get, path = "/creatives/{id}", responses((status = 200, body = Creative)))]
async fn get_creative(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    if let Some(c) = store.creatives.read().get(&id).cloned() {
        Json(c).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(put, path = "/creatives/{id}", request_body = Creative, responses((status = 200, body = Creative)))]
async fn update_creative(State(store): State<Arc<Store>>, Path(id): Path<String>, Json(c): Json<Creative>) -> impl axum::response::IntoResponse {
    let mut map = store.creatives.write();
    if map.contains_key(&id) {
        map.insert(id.clone(), c.clone());
        Json(c).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(delete, path = "/creatives/{id}", responses((status = 204)))]
async fn delete_creative(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    store.creatives.write().remove(&id);
    axum::http::StatusCode::NO_CONTENT
}

#[utoipa::path(post, path = "/metrics", request_body = Metric, responses((status = 200, body = Metric)))]
async fn create_metric(State(store): State<Arc<Store>>, Json(m): Json<Metric>) -> impl axum::response::IntoResponse {
    store.metrics.write().insert(m.id.clone(), m.clone());
    Json(m)
}

#[utoipa::path(get, path = "/metrics/{id}", responses((status = 200, body = Metric)))]
async fn get_metric(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    if let Some(m) = store.metrics.read().get(&id).cloned() {
        Json(m).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(put, path = "/metrics/{id}", request_body = Metric, responses((status = 200, body = Metric)))]
async fn update_metric(State(store): State<Arc<Store>>, Path(id): Path<String>, Json(m): Json<Metric>) -> impl axum::response::IntoResponse {
    let mut map = store.metrics.write();
    if map.contains_key(&id) {
        map.insert(id.clone(), m.clone());
        Json(m).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(delete, path = "/metrics/{id}", responses((status = 204)))]
async fn delete_metric(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    store.metrics.write().remove(&id);
    axum::http::StatusCode::NO_CONTENT
}

#[utoipa::path(post, path = "/playbooks", request_body = Playbook, responses((status = 200, body = Playbook)))]
async fn create_playbook(State(store): State<Arc<Store>>, Json(p): Json<Playbook>) -> impl axum::response::IntoResponse {
    store.playbooks.write().insert(p.id.clone(), p.clone());
    Json(p)
}

#[utoipa::path(get, path = "/playbooks/{id}", responses((status = 200, body = Playbook)))]
async fn get_playbook(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    if let Some(p) = store.playbooks.read().get(&id).cloned() {
        Json(p).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(put, path = "/playbooks/{id}", request_body = Playbook, responses((status = 200, body = Playbook)))]
async fn update_playbook(State(store): State<Arc<Store>>, Path(id): Path<String>, Json(p): Json<Playbook>) -> impl axum::response::IntoResponse {
    let mut map = store.playbooks.write();
    if map.contains_key(&id) {
        map.insert(id.clone(), p.clone());
        Json(p).into_response()
    } else {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

#[utoipa::path(delete, path = "/playbooks/{id}", responses((status = 204)))]
async fn delete_playbook(State(store): State<Arc<Store>>, Path(id): Path<String>) -> impl axum::response::IntoResponse {
    store.playbooks.write().remove(&id);
    axum::http::StatusCode::NO_CONTENT
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_campaign,
        get_campaign,
        update_campaign,
        delete_campaign,
        create_creative,
        get_creative,
        update_creative,
        delete_creative,
        create_metric,
        get_metric,
        update_metric,
        delete_metric,
        create_playbook,
        get_playbook,
        update_playbook,
        delete_playbook
    ),
    components(schemas(Campaign, Creative, Metric, Playbook))
)]
struct ApiDoc;

pub async fn serve() -> anyhow::Result<()> {
    let store = Arc::new(Store::default());

    let grpc_service = MyMarketingService { store: store.clone() };
    let grpc = GrpcServer::builder()
        .add_service(proto::marketing_service_server::MarketingServiceServer::new(grpc_service));

    let app = Router::new()
        .route("/campaigns", post(create_campaign))
        .route("/campaigns/:id", get(get_campaign).put(update_campaign).delete(delete_campaign))
        .route("/creatives", post(create_creative))
        .route("/creatives/:id", get(get_creative).put(update_creative).delete(delete_creative))
        .route("/metrics", post(create_metric))
        .route("/metrics/:id", get(get_metric).put(update_metric).delete(delete_metric))
        .route("/playbooks", post(create_playbook))
        .route("/playbooks/:id", get(get_playbook).put(update_playbook).delete(delete_playbook))
        .merge(SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(store.clone());

    tokio::spawn(async move {
        grpc.serve(([127, 0, 0, 1], 50051).into()).await.unwrap();
    });

    let addr = "127.0.0.1:8080".parse().unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub struct MyMarketingService {
    store: Arc<Store>,
}

#[tonic::async_trait]
impl proto::marketing_service_server::MarketingService for MyMarketingService {
    async fn create_campaign(&self, request: Request<proto::Campaign>) -> Result<Response<proto::Campaign>, Status> {
        let c: Campaign = request.into_inner().into();
        self.store.campaigns.write().insert(c.id.clone(), c.clone());
        Ok(Response::new(c.into()))
    }

    async fn get_campaign(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Campaign>, Status> {
        let id = request.into_inner().id;
        match self.store.campaigns.read().get(&id) {
            Some(c) => Ok(Response::new(c.clone().into())),
            None => Err(Status::not_found("not found")),
        }
    }

    async fn update_campaign(&self, request: Request<proto::Campaign>) -> Result<Response<proto::Campaign>, Status> {
        let c: Campaign = request.into_inner().into();
        self.store.campaigns.write().insert(c.id.clone(), c.clone());
        Ok(Response::new(c.into()))
    }

    async fn delete_campaign(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Empty>, Status> {
        let id = request.into_inner().id;
        self.store.campaigns.write().remove(&id);
        Ok(Response::new(proto::Empty {}))
    }

    async fn create_creative(&self, request: Request<proto::Creative>) -> Result<Response<proto::Creative>, Status> {
        let c: Creative = request.into_inner().into();
        self.store.creatives.write().insert(c.id.clone(), c.clone());
        Ok(Response::new(c.into()))
    }
    async fn get_creative(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Creative>, Status> {
        let id = request.into_inner().id;
        match self.store.creatives.read().get(&id) {
            Some(c) => Ok(Response::new(c.clone().into())),
            None => Err(Status::not_found("not found")),
        }
    }
    async fn update_creative(&self, request: Request<proto::Creative>) -> Result<Response<proto::Creative>, Status> {
        let c: Creative = request.into_inner().into();
        self.store.creatives.write().insert(c.id.clone(), c.clone());
        Ok(Response::new(c.into()))
    }
    async fn delete_creative(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Empty>, Status> {
        let id = request.into_inner().id;
        self.store.creatives.write().remove(&id);
        Ok(Response::new(proto::Empty {}))
    }

    async fn create_metric(&self, request: Request<proto::Metric>) -> Result<Response<proto::Metric>, Status> {
        let m: Metric = request.into_inner().into();
        self.store.metrics.write().insert(m.id.clone(), m.clone());
        Ok(Response::new(m.into()))
    }
    async fn get_metric(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Metric>, Status> {
        let id = request.into_inner().id;
        match self.store.metrics.read().get(&id) {
            Some(m) => Ok(Response::new(m.clone().into())),
            None => Err(Status::not_found("not found")),
        }
    }
    async fn update_metric(&self, request: Request<proto::Metric>) -> Result<Response<proto::Metric>, Status> {
        let m: Metric = request.into_inner().into();
        self.store.metrics.write().insert(m.id.clone(), m.clone());
        Ok(Response::new(m.into()))
    }
    async fn delete_metric(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Empty>, Status> {
        let id = request.into_inner().id;
        self.store.metrics.write().remove(&id);
        Ok(Response::new(proto::Empty {}))
    }

    async fn create_playbook(&self, request: Request<proto::Playbook>) -> Result<Response<proto::Playbook>, Status> {
        let p: Playbook = request.into_inner().into();
        self.store.playbooks.write().insert(p.id.clone(), p.clone());
        Ok(Response::new(p.into()))
    }
    async fn get_playbook(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Playbook>, Status> {
        let id = request.into_inner().id;
        match self.store.playbooks.read().get(&id) {
            Some(p) => Ok(Response::new(p.clone().into())),
            None => Err(Status::not_found("not found")),
        }
    }
    async fn update_playbook(&self, request: Request<proto::Playbook>) -> Result<Response<proto::Playbook>, Status> {
        let p: Playbook = request.into_inner().into();
        self.store.playbooks.write().insert(p.id.clone(), p.clone());
        Ok(Response::new(p.into()))
    }
    async fn delete_playbook(&self, request: Request<proto::IdRequest>) -> Result<Response<proto::Empty>, Status> {
        let id = request.into_inner().id;
        self.store.playbooks.write().remove(&id);
        Ok(Response::new(proto::Empty {}))
    }
}
