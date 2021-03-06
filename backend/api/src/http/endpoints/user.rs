use crate::extractor::{
    reply_signin_auth, FirebaseUser, WrapAuthClaimsCookieDbNoCsrf, WrapAuthClaimsNoDb,
};
use crate::{
    db::{self, user::register},
    error,
};
use actix_web::HttpResponse;
use core::settings::RuntimeSettings;
use jsonwebtoken as jwt;
use paperclip::actix::{
    api_v2_operation,
    web::{Data, Json, Query, ServiceConfig},
};
use shared::{
    api::endpoints::{
        user::{Profile, Register, Signin, SingleSignOn, UserLookup},
        ApiEndpoint,
    },
    domain::{
        auth::{AuthClaims, RegisterRequest, RegisterSuccess, SigninSuccess, SingleSignOnSuccess},
        user::UserLookupQuery,
    },
    error::auth::RegisterErrorKind,
};
use sqlx::PgPool;

/// Lookup a user.
#[api_v2_operation]
async fn user_lookup(
    db: Data<PgPool>,
    query: Query<UserLookupQuery>,
) -> Result<Json<<UserLookup as ApiEndpoint>::Res>, error::UserNotFound> {
    let query = query.into_inner();

    if query.id.is_none() && query.firebase_id.is_none() && query.name.is_none() {
        return Err(error::UserNotFound::UserNotFound);
    }

    db::user::lookup(
        db.as_ref(),
        query.id,
        query.firebase_id.as_deref(),
        query.name.as_deref(),
    )
    .await?
    .map(Json)
    .ok_or(error::UserNotFound::UserNotFound)
}

/// Login with a user.
#[api_v2_operation]
async fn handle_signin_credentials(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    user: FirebaseUser,
) -> Result<HttpResponse, error::UserNotFound> {
    let user_id = db::user::firebase_to_id(&db, &user.id)
        .await?
        .ok_or(error::UserNotFound::UserNotFound)?;

    let (csrf, cookie) =
        reply_signin_auth(user_id, &settings.jwt_encoding_key, settings.is_local())?;

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(SigninSuccess { csrf }))
}

async fn validate_register_req(req: &RegisterRequest) -> Result<(), error::Register> {
    // todo: decide if we should check for an _empty_ email?
    if req.username.is_empty() {
        return Err(error::Register::RegisterError(
            RegisterErrorKind::EmptyDisplayName,
        ));
    }

    Ok(())
}

/// Register a new user.
#[api_v2_operation]
async fn handle_register(
    settings: Data<RuntimeSettings>,
    db: Data<PgPool>,
    user: FirebaseUser,
    req: Json<RegisterRequest>,
) -> actix_web::Result<HttpResponse, error::Register> {
    validate_register_req(&req).await?;

    let id = register(db.as_ref(), &user.id, &req).await?;

    let (csrf, cookie) = reply_signin_auth(id, &settings.jwt_encoding_key, settings.is_local())?;

    Ok(HttpResponse::Created()
        .cookie(cookie)
        .json(RegisterSuccess::Signin(csrf)))
}

/// Get a user by their profile.
#[api_v2_operation]
async fn handle_get_profile(
    db: Data<PgPool>,
    claims: WrapAuthClaimsNoDb,
) -> Result<Json<<Profile as ApiEndpoint>::Res>, error::UserNotFound> {
    // todo: figure out how to do `<Profile as ApiEndpoint>::Err`

    db::user::profile(db.as_ref(), claims.0.id)
        .await?
        .map(Json)
        .ok_or(error::UserNotFound::UserNotFound)
}

/// Sign in as a user via SSO.
#[api_v2_operation]
async fn handle_authorize(
    settings: Data<RuntimeSettings>,
    auth: WrapAuthClaimsCookieDbNoCsrf,
) -> Result<Json<<SingleSignOn as ApiEndpoint>::Res>, error::Server> {
    let claims = AuthClaims {
        id: auth.0.id,
        csrf: None,
    };

    let jwt = jwt::encode(&jwt::Header::default(), &claims, &settings.jwt_encoding_key)?;

    Ok(Json(SingleSignOnSuccess { jwt }))
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        Profile::PATH,
        Profile::METHOD.route().to(handle_get_profile),
    )
    .route(
        SingleSignOn::PATH,
        SingleSignOn::METHOD.route().to(handle_authorize),
    )
    .route(Register::PATH, Register::METHOD.route().to(handle_register))
    .route(
        Signin::PATH,
        Signin::METHOD.route().to(handle_signin_credentials),
    )
    .route(UserLookup::PATH, UserLookup::METHOD.route().to(user_lookup));
}
