//! Handlers for collection-namespaced content resources.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    api::AppState,
    errors::ApiError,
    models::content::Content,
    services::content::{self, FilterField, FilterExpr, FilterOp, ListOptions, MetaField, SortField, SortSpec},
    utils::validate_identifier,
};

#[derive(Debug, Default, Deserialize)]
pub struct ListQuery {
    #[serde(rename = "where")]
    pub filters: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl ListQuery {
    fn into_options(self) -> Result<ListOptions, ApiError> {
        let mut options = ListOptions::default();
        options.limit = self.limit.unwrap_or(20);
        options.offset = self.offset.unwrap_or(0);

        options.filters = self
            .filters
            .as_deref()
            .unwrap_or("")
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|expr| parse_filter_expr(expr.trim()))
            .collect::<Result<Vec<_>, _>>()?;

        options.sort = match self.sort {
            Some(sort) => Some(parse_sort_expr(&sort)?),
            None => None,
        };

        Ok(options)
    }
}

pub async fn create(
    State(state): State<AppState>,
    Path(collection): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<Content>, ApiError> {
    let item = content::create(&state.pool, &collection, payload).await?;
    Ok(Json(item))
}

pub async fn list(
    State(state): State<AppState>,
    Path(collection): Path<String>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<Content>>, ApiError> {
    let options = query.into_options()?;
    let items = content::list(&state.pool, &collection, options).await?;
    Ok(Json(items))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
) -> Result<Json<Content>, ApiError> {
    let item = content::get(&state.pool, &collection, id).await?;
    Ok(Json(item))
}

pub async fn delete(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
) -> Result<Json<Value>, ApiError> {
    content::delete(&state.pool, &collection, id).await?;
    Ok(Json(serde_json::json!({ "deleted": id })))
}

pub async fn update(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
    Json(payload): Json<Value>,
) -> Result<Json<Content>, ApiError> {
    let item = content::update(&state.pool, &collection, id, payload).await?;
    Ok(Json(item))
}

pub async fn patch(
    State(state): State<AppState>,
    Path((collection, id)): Path<(String, Uuid)>,
    Json(payload): Json<Value>,
) -> Result<Json<Content>, ApiError> {
    let item = content::patch(&state.pool, &collection, id, payload).await?;
    Ok(Json(item))
}

fn parse_filter_expr(expr: &str) -> Result<FilterExpr, ApiError> {
    let mut parts = expr.splitn(3, ':');
    let field = parts
        .next()
        .ok_or_else(|| ApiError::BadRequest("missing filter field".into()))?;
    let op = parts
        .next()
        .ok_or_else(|| ApiError::BadRequest("missing filter operator".into()))?;
    let value = parts
        .next()
        .ok_or_else(|| ApiError::BadRequest("missing filter value".into()))?;

    Ok(FilterExpr {
        field: parse_field(field)?,
        op: parse_op(op)?,
        value: value.to_string(),
    })
}

fn parse_sort_expr(expr: &str) -> Result<SortSpec, ApiError> {
    let (raw_field, desc) = if let Some(rest) = expr.strip_prefix('-') {
        (rest, true)
    } else if let Some(rest) = expr.strip_suffix(".desc") {
        (rest, true)
    } else if let Some(rest) = expr.strip_suffix(".asc") {
        (rest, false)
    } else {
        (expr, false)
    };

    Ok(SortSpec {
        field: parse_sort_field(raw_field)?,
        desc,
    })
}

fn parse_field(field: &str) -> Result<FilterField, ApiError> {
    if let Some(rest) = field.strip_prefix("data.") {
        validate_identifier(rest)?;
        return Ok(FilterField::Data(rest.to_string()));
    }

    match field {
        "id" => Ok(FilterField::Meta(MetaField::Id)),
        "created_at" => Ok(FilterField::Meta(MetaField::CreatedAt)),
        "updated_at" => Ok(FilterField::Meta(MetaField::UpdatedAt)),
        other => {
            validate_identifier(other)?;
            Ok(FilterField::Data(other.to_string()))
        }
    }
}

fn parse_sort_field(field: &str) -> Result<SortField, ApiError> {
    if let Some(rest) = field.strip_prefix("data.") {
        validate_identifier(rest)?;
        return Ok(SortField::Data(rest.to_string()));
    }

    match field {
        "id" => Ok(SortField::Meta(MetaField::Id)),
        "created_at" => Ok(SortField::Meta(MetaField::CreatedAt)),
        "updated_at" => Ok(SortField::Meta(MetaField::UpdatedAt)),
        other => {
            validate_identifier(other)?;
            Ok(SortField::Data(other.to_string()))
        }
    }
}

fn parse_op(op: &str) -> Result<FilterOp, ApiError> {
    match op {
        "eq" => Ok(FilterOp::Eq),
        "ne" => Ok(FilterOp::Ne),
        "contains" => Ok(FilterOp::Contains),
        "gt" => Ok(FilterOp::Gt),
        "gte" => Ok(FilterOp::Gte),
        "lt" => Ok(FilterOp::Lt),
        "lte" => Ok(FilterOp::Lte),
        _ => Err(ApiError::BadRequest(format!("unsupported operator: {op}"))),
    }
}
