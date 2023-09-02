use async_graphql::{Context, FieldResult, InputObject};
use dotenv::dotenv;
// use async_graphql::parser::Error;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema, SimpleObject,
};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use http::StatusCode;
use kv::{KVStore, RocksDB};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

mod kv;
mod utils;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[graphql(visible = false)]
    pub password: String,
}

#[derive(InputObject, Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject, Deserialize)]
pub struct CodeOrTextGenerationInput {
    pub prompt: String,
    pub user: String,
    limit: Option<i32>,
    previous_prompt: Option<String>,
    provious_prompt_response: Option<String>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct LoginAndRegiterResponse {
    pub token: String,
    pub user: User,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct CodeOrTextGenerationResponse {
    pub response: String,
    pub user: User,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct CodeOrTextGenerationToSave {
    pub response: String,
    pub prompt: String,
    pub time: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: String,
    pub email: String,
    pub first_message: String,
}

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn register_user(
        &self,
        ctx: &Context<'_>,
        user_input: UserInput,
    ) -> FieldResult<LoginAndRegiterResponse> {
        let db = ctx.data::<RocksDB>()?;

        // first check if same email exists

        let user = db.find_unique(&user_input.email);

        if user.is_some() {
            return Err("User already exists".into());
        }

        // generate random integer
        let mut rng = rand::thread_rng().to_owned();

        let user_to_save_to_kv = User {
            id: rng.gen::<u32>().to_string(),
            name: user_input.name,
            email: user_input.email,
            password: utils::password::hash_password(&user_input.password),
        };

        let saved_user = db.save(
            &user_to_save_to_kv.email,
            &serde_json::to_string(&user_to_save_to_kv).unwrap(),
        );

        if saved_user {
            let token = utils::jwt::create_jwt(&user_to_save_to_kv.email);
            let response = LoginAndRegiterResponse {
                token,
                user: user_to_save_to_kv,
            };
            Ok(response)
        } else {
            Err("Error saving user".into())
        }
    }

    async fn login_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> FieldResult<LoginAndRegiterResponse> {
        let db = ctx.data::<RocksDB>()?;
        let user = db.find_unique(&email);
        match user {
            Some(user) => {
                let user: User = serde_json::from_str(&user).unwrap();
                let is_valid_password = utils::password::verify_password(&password, &user.password);
                if is_valid_password {
                    let token = utils::jwt::create_jwt(&user.email);
                    let response = LoginAndRegiterResponse { token, user };
                    Ok(response)
                } else {
                    Err("Invalid password".into())
                }
            }
            None => Err("User not found".into()),
        }
    }

    async fn create_chat(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> FieldResult<LoginAndRegiterResponse> {
        let db = ctx.data::<RocksDB>()?;
        let user = db.find_unique(&email);
        match user {
            Some(user) => {
                let user: User = serde_json::from_str(&user).unwrap();
                let is_valid_password = utils::password::verify_password(&password, &user.password);
                if is_valid_password {
                    let token = utils::jwt::create_jwt(&user.email);
                    let response = LoginAndRegiterResponse { token, user };
                    Ok(response)
                } else {
                    Err("Invalid password".into())
                }
            }
            None => Err("User not found".into()),
        }
    }
}

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn get_user(&self, ctx: &Context<'_>, id: String) -> FieldResult<User> {
        let db = ctx.data::<RocksDB>()?;
        let user = db.find_unique(&id);
        match user {
            Some(user) => {
                let user: User = serde_json::from_str(&user).unwrap();
                Ok(user)
            }
            None => Err("User not found".into()),
        }
    }

    async fn get_users(&self, ctx: &Context<'_>) -> FieldResult<Vec<User>> {
        let db = ctx.data::<RocksDB>()?;
        let mut users: Vec<User> = Vec::new();
        let users_from_db = db.find();
        for user in users_from_db {
            let user: User = serde_json::from_str(&user).unwrap();
            users.push(user);
        }
        Ok(users)
    }

    async fn generate_code_or_text(
        &self,
        ctx: &Context<'_>,
        code_or_text_generation_input: CodeOrTextGenerationInput,
    ) -> FieldResult<CodeOrTextGenerationResponse> {
        let db = ctx.data::<RocksDB>()?;
        let user = db.find_unique(&code_or_text_generation_input.user);
        match user {
            Some(user) => {
                let user1: User = serde_json::from_str(&user).unwrap();
                let email = &user1.email;
                let name = &user1.name;

                let string_key = format!("messages:{}:{}", name, email);

                // get the previous prompt and previous prompt response from input
                let previous_prompt = code_or_text_generation_input.previous_prompt;
                let previous_prompt_response =
                    code_or_text_generation_input.provious_prompt_response;
                let prompt = &code_or_text_generation_input.prompt;
                // concat the previous prompt and the previous prompt response
                let combined_string: String = previous_prompt.unwrap_or_else(|| String::new())
                    + &previous_prompt_response.unwrap_or(String::new())
                    + &prompt;

                let generated_code_or_text: String =
                    utils::gpt::generate_code_or_text(combined_string).await?;
                let code_or_text_generation_response = CodeOrTextGenerationResponse {
                    response: generated_code_or_text.as_str().to_string(),
                    user: user1,
                };

                let code_or_text_generation_to_save = CodeOrTextGenerationToSave {
                    response: generated_code_or_text.to_string(),
                    prompt: code_or_text_generation_input.prompt,
                    time: chrono::Local::now().to_string(),
                };

                let code_or_text_generation_to_save =
                    serde_json::to_string(&code_or_text_generation_to_save).unwrap();

                // save to db
                let _saved = db.save_many(&string_key, &code_or_text_generation_to_save);
                Ok(code_or_text_generation_response)
            }
            None => Err("Unable to generate text or code".into()),
        }
    }

    // // get all code or text generated by a user
    async fn get_code_or_text_generated_by_user(
        &self,
        ctx: &Context<'_>,
        user: String,
    ) -> FieldResult<Vec<CodeOrTextGenerationToSave>> {
        let db = ctx.data::<RocksDB>()?;
        let string_key = format!("messages:{}", &user);
        let code_or_text_generated_by_user = db.find_unique(&string_key);

        match code_or_text_generated_by_user {
            Some(code_or_text) => {
                // First deserialize into Vec<String>
                let vec: Vec<String> = serde_json::from_str(&code_or_text).unwrap();

                // Then deserialize each string into CodeOrTextGenerationToSave
                let result: Vec<CodeOrTextGenerationToSave> = vec
                    .into_iter()
                    .map(|s| serde_json::from_str(&s))
                    .collect::<Result<Vec<_>, _>>() // Collect and handle any errors from serde_json::from_str
                    .unwrap(); // unwrap or better yet, handle the error

                Ok(result)
            }
            None => Err("Unable to get code or text generated by user".into()),
        }
    }
}

// non blocking using threads
#[tokio::main]
async fn main() {
    dotenv().ok();
    // make kv connection global
    let db: kv::RocksDB = kv::KVStore::init("/tmp/rocks/test-graph-001");
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(db.clone())
        .finish();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    println!("Playground: http://localhost:5001");

    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, request): (
            Schema<Query, Mutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            // Spawn the execution onto a separate task
            let result = tokio::spawn(async move { schema.execute(request).await })
                .await
                .unwrap();
            // .unwrap_or_else(|_| async_graphql::Response::from_errors("Task failed"));
            Ok::<_, Infallible>(GraphQLResponse::from(result))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes =
        graphql_playground
            .or(graphql_post.with(cors))
            .recover(|err: Rejection| async move {
                if let Some(GraphQLBadRequest(err)) = err.find() {
                    return Ok::<_, Infallible>(warp::reply::with_status(
                        err.to_string(),
                        StatusCode::BAD_REQUEST,
                    ));
                }

                Ok(warp::reply::with_status(
                    "INTERNAL_SERVER_ERROR".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            });

    warp::serve(routes).run(([0, 0, 0, 0], 5001)).await;
}
