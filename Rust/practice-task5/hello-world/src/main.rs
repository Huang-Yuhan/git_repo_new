use actix_web::{web, App, HttpResponse, HttpServer, Responder, error};
use serde::{Deserialize, Serialize};
use actix_web::error::Error;
use sqlx::mysql::MySqlPool;

#[derive(Debug, Deserialize)]
struct AddRequest {
    a: i32,
    b: i32,
}

#[derive(Debug, Serialize)]
struct AddResponse {
    res: i32,
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

async fn add(req: Result<web::Json<AddRequest>, Error>) -> impl Responder {
    match req {
        Ok(json) => {
            let res = AddResponse {
                res: json.a + json.b,
            };
            HttpResponse::Ok().json(res)
        }
        Err(err) => {
            if let Some(json_err) = err.as_error::<error::JsonPayloadError>() {
                match json_err {
                    error::JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body("Unsupported media type"),
                    _ => HttpResponse::BadRequest().finish()
                }
            } else {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_pool = MySqlPool::connect("mysql://root:20180203loveyz@localhost:3306/actix_db")
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move|| {
        App::new()
            .app_data(db_pool.clone())
            .route("/", web::get().to(index))
            .route("/add", web::post().to(add))
            .service(UserService::login)
            .service(MovieService::get_movie_by_name)
            .service(CommentService::comment_movie)
    })
    .bind("127.0.0.1:8080")? // 可更改监听端口
    .run()
    .await
}

mod UserService
{
    use actix_web::{web,post, App, HttpResponse, HttpServer, Responder, error};
    use serde::{Deserialize, Serialize};
    use actix_web::error::Error;
    use sqlx::{mysql::MySqlPool, MySql};


    #[derive(Debug, Deserialize)]
    pub struct LoginRequest {
        username: String,
        password: String,
    }

    #[derive(Debug, Serialize,sqlx::FromRow)]
    pub struct User{
        id:i32,
        username:String,
        password:String,
        register_time:String
    }

    #[post("/login")]
    pub async fn login(req: Result<web::Json<LoginRequest>, Error>,db_pool:web::Data<MySqlPool>) -> impl Responder {
        return HttpResponse::Ok().json("登录成功");
        match req {
            Ok(json) => {
                let LoginForm = LoginRequest {
                    username: json.username.clone(),
                    password: json.password.clone(),
                };
                //在数据库里面查找用户返回
                let query_result = sqlx::query_as::<MySql,User>(
                    "SELECT * FROM user_info WHERE username = ? AND password = ?",
                ).bind(LoginForm.username).bind(LoginForm.password).fetch_optional(db_pool.get_ref()).await;
                
                match query_result {
                    Ok(Some(user)) => {
                        HttpResponse::Ok().json(user)
                    }
                    Ok(None) => {
                        HttpResponse::Ok().json("用户名或密码错误")
                    }
                    Err(err) => {
                        HttpResponse::InternalServerError().body("出错了")
                    }
                }
            }
            Err(err) => {
                if let Some(json_err) = err.as_error::<error::JsonPayloadError>() {
                    match json_err {
                        error::JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body("Unsupported media type"),
                        _ => HttpResponse::BadRequest().finish()
                    }
                } else {
                    HttpResponse::InternalServerError().body("Internal Server Error")
                }
            }
        }
    }
}

mod MovieService
{
    use actix_web::{web,get, App, HttpResponse, HttpServer, Responder, error};
    use serde::{Deserialize, Serialize};
    use actix_web::error::Error;
    use sqlx::mysql::MySqlPool;
    use sqlx::{Pool, MySql};

    #[derive(Debug, Serialize,sqlx::FromRow)]
    pub struct Movie{
        movie_id:i32,
        movie_name:String,
        release_time:String,
        rate:f32,
        description:String,
    }

    #[get("/movie?movie_name={movie_name}")]
    pub async fn get_movie_by_name(movie_name:web::Query<String>,db_pool:web::Data<MySqlPool>) -> impl Responder {
        let query_result = sqlx::query_as::<MySql,Movie>(
            "SELECT * FROM movie_info WHERE movie_name = ?",
        ).bind(movie_name.into_inner()).fetch_optional(db_pool.get_ref()).await;
        
        match query_result {
            Ok(Some(movie)) => {
                HttpResponse::Ok().json(movie)
            }
            Ok(None) => {
                HttpResponse::Ok().json("没有找到该电影")
            }
            Err(err) => {
                HttpResponse::InternalServerError().body("Internal Server Error")
            }
        }
    }
}

mod CommentService
{
    use actix_web::{web,post, App, HttpResponse, HttpServer, Responder, error};
    use serde::{Deserialize, Serialize};
    use actix_web::error::Error;
    use sqlx::mysql::MySqlPool;
    use sqlx::{Pool, MySql};


    #[derive(Debug, Deserialize)]
    pub struct CommentRequest {
        movie_name: String,
        commentator: String,
        comment_content: String,
        rate: f32,
    }

    #[derive(Debug, Serialize,sqlx::FromRow)]
    pub struct Comment{
        comment_id:i32,
        movie_name:String,
        commentator:String,
        comment_content:String,
        rate:f32,
    }

    #[post("/comment")]
    pub async fn comment_movie(req: Result<web::Json<CommentRequest>, Error>,db_pool:web::Data<MySqlPool>) -> impl Responder {
        match req {
            Ok(json) => {
                let comment_form = CommentRequest {
                    movie_name: json.movie_name.clone(),
                    commentator: json.commentator.clone(),
                    comment_content: json.comment_content.clone(),
                    rate: json.rate.clone(),
                };
                //在数据库里面查找用户返回
                let query_result = sqlx::query::<MySql>(
                    "INSERT INTO comment_info (movie_name,commentator,comment_content,rate) VALUES (?,?,?,?)",
                ).bind(comment_form.movie_name.clone()).bind(comment_form.commentator.clone()).bind(comment_form.comment_content.clone()).bind(comment_form.rate).execute(db_pool.get_ref()).await;
                
                match query_result {
                    Ok(_) => {
                        //然后去修改电影的评分
                        //在数据库里面查找所有关于这部电影的评分
                        let query_result = sqlx::query_as::<MySql,Comment>(
                            "SELECT * FROM comment_info WHERE movie_name = ?",
                        ).bind(comment_form.movie_name.clone()).fetch_all(db_pool.get_ref()).await;

                        let mut sum:f32 = 0.0;
                        let mut count:i32 = 0;
                        for comment in query_result.unwrap(){
                            sum += comment.rate;
                            count += 1;
                        }

                        let query_movie_result = sqlx::query::<MySql>(
                            "UPDATE movie_info SET rate = ? WHERE movie_name = ?",
                        ).bind(sum/count as f32).bind(comment_form.movie_name.clone()).execute(db_pool.get_ref()).await;
                        HttpResponse::Ok().json("评论成功")

                    }
                    Err(err) => {
                        HttpResponse::InternalServerError().body("Internal Server Error")
                    }
                }
            }
            Err(err) => {
                if let Some(json_err) = err.as_error::<error::JsonPayloadError>() {
                    match json_err {
                        error::JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body("Unsupported media type"),
                        _ => HttpResponse::BadRequest().finish()
                    }
                } else {
                    HttpResponse::InternalServerError().body("Internal Server Error")
                }
            }
        }
    }


}