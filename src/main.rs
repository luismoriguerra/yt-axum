use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::{delete, get}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub icon: String,
}

impl Category {
    pub fn new() -> Category {
        Default::default()
    }
}

#[derive(Debug, OpenApi)]
#[openapi(
    paths(
        get_all_categories,
        create_new_category,
        delete_category
    ),
    components(schemas(Category),),
    tags(
        (name = "Category", description = "Category operations"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    println!("starting server on port 8081");

    let app = Router::new()
        .route(
            "/category",
            get(get_all_categories).post(create_new_category),
        )
        .route("/category/:id", delete(delete_category))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}

#[utoipa::path(
    get,
    path = "/category",
    responses(
        (status=200,body=[Category]),
        (status=404)
    )
)]
async fn get_all_categories() -> Json<Vec<Category>> {
    Json(vec![
        Category {
            id: 1,
            name: "Category 1".to_string(),
            url: "http://example.com".to_string(),
            icon: "icon1".to_string(),
        },
        Category {
            id: 2,
            name: "Category 2".to_string(),
            url: "http://example.com".to_string(),
            icon: "icon2".to_string(),
        },
    ])
}

#[utoipa::path(
    post,
    path = "/category",
    responses(
        (status=200,body=Category, description="Category created"),
        (status=404, description="Category not found")
    )
)]
async fn create_new_category(Json(category): Json<Category>) -> Json<Category> {
    let modify = Category { id: 55, ..category };

    Json(modify)
}

#[utoipa::path(
    delete,
    path = "/category/:id",
    responses(
        (status=200,body=Category, description="Category deleted"),
        (status=404, description="Category not found")
    )
)]
async fn delete_category(Path(id): Path<usize>) -> impl IntoResponse {
    if id == 8081 {
        (StatusCode::OK, Json(true)).into_response()
    } else {
        (StatusCode::NOT_FOUND, Json(false)).into_response()
    }
}
