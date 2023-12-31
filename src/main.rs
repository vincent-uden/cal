use actix_files::Files;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::http::header::ContentType;
use actix_web::{
    get, post, web, App, HttpResponse, HttpServer, Responder,
    Result as AwResult,
};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use diesel::{
    r2d2, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection,
    TextExpressionMethods,
};
use maud::{html, Markup};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Deserialize;
use track_notes::models::{Food, User};
use track_notes::schema::{foods, users};
use ui::layout;

use crate::ui::{food_creator, food_searcher, sign_in_page, sign_up_page};

mod ui;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

struct AppState<'a> {
    tracks: Vec<(&'a str, &'a str)>,
    db: DbPool,
}

fn markup_to_resp(m: Markup) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(m.into_string())
}

fn redirect(path: &str) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .insert_header(("HX-Redirect", path))
        .body("No session")
}

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct CreateFoodData {
    name: String,
    calories: i32,
    protein: i32,
}

#[derive(Deserialize)]
struct SearchData {
    #[serde(alias = "search-name")]
    search_name: String,
}

struct SessionData {
    authenticated: bool,
    user_id: i32,
}

impl SessionData {
    pub fn new(authenticated: bool, user_id: i32) -> SessionData {
        SessionData {
            authenticated,
            user_id,
        }
    }

    pub fn from_session(session: &Session) -> Option<SessionData> {
        let entries = session.entries();

        if entries.contains_key("authenticated") && entries.contains_key("userId") {
            Some(SessionData::new(
                entries.get("authenticated").unwrap() == "true",
                entries.get("userId").unwrap().parse().unwrap(),
            ))
        } else {
            None
        }
    }
}

impl AppState<'_> {
    pub fn new(db: DbPool) -> AppState<'static> {
        AppState {
            tracks: vec![
                ("Abu Dhabi", "/tracks/abudhabi.svg"),
                ("Australia", "/tracks/australia.svg"),
                ("Austria", "/tracks/austria.svg"),
                ("Azerbaijan", "/tracks/azerbaijan.svg"),
                ("Bahrain", "/tracks/bahrain.svg"),
                ("Belgium", "/tracks/belgium.svg"),
                ("Brazil", "/tracks/brazil.svg"),
                ("Canada", "/tracks/canada.svg"),
                ("China", "/tracks/china.svg"),
                ("France", "/tracks/france.svg"),
                ("Great Britain", "/tracks/greatbritain.svg"),
                ("Hungary", "/tracks/hungary.svg"),
                ("Italy", "/tracks/italy.svg"),
                ("Japan", "/tracks/japan.svg"),
                ("Mexico", "/tracks/mexico.svg"),
                ("Monaco", "/tracks/monaco.svg"),
                ("Netherlands", "/tracks/netherlands.svg"),
                ("Russia", "/tracks/russia.svg"),
                ("Singapore", "/tracks/singapore.svg"),
                ("Spain", "/tracks/spain.svg"),
                ("Usa", "/tracks/usa.svg"),
                ("Vietnam", "/tracks/vietnam.svg"),
            ],
            db,
        }
    }
}

#[get("/")]
async fn index() -> AwResult<Markup> {
    Ok(layout(sign_in_page(None)))
}

#[get("/sign-up")]
async fn sign_up() -> AwResult<Markup> {
    Ok(layout(sign_up_page(None)))
}

#[get("/notes")]
async fn notes(data: web::Data<AppState<'static>>) -> impl Responder {
    let html2 = layout(html! {
        div class="h-8" {}
        h1 class="text-4xl" {
            "Track Notes"
        }
        div class="h-4" {}

        div class="flex flex-row flex-wrap gap-4" {
            @for (track_name, _) in &data.tracks {
                button class="px-4 py-2 bg-sky-500 rounded-lg hover:bg-sky-400" hx-post=(format!("/change_track/{}", track_name)) hx-target="#track-container" {
                    (track_name)
                }
            }
        }

        div class="h-8" {}

        div id="track-container" class="flex flex-col justify-center" {}
    });

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html2.into_string())
}

#[post("/change_track/{track_name}")]
async fn change_track(
    url: web::Path<String>,
    data: web::Data<AppState<'static>>,
) -> impl Responder {
    let mut file_path = "";
    let mut track_name = "";
    let url_path = url.into_inner();
    for (name, path) in &data.tracks {
        if &url_path == name {
            file_path = path;
            track_name = name;
        }
    }

    let html2 = html! {
        div class="flex flex-row justify-stretch" {
            div class="grow" {
                p class="text-2xl" { (track_name) }
                div class="h-4" {}
                img src=(file_path) class="max-h-72" {}
            }
            div class="flex flex-col"{
                label for="pb" {"Personal Best"}
                input id="pb" class="bg-zinc-800 px-4 py-2" placeholder="0:00.00" {}
            }
        }
    };

    HttpResponse::Ok().body(html2.into_string())
}

#[get("/meal_builder")]
async fn meal_builder(
    data: web::Data<AppState<'static>>,
    session: Session,
) -> AwResult<HttpResponse> {
    match SessionData::from_session(&session) {
        Some(session_data) => Ok(markup_to_resp(layout(html! {
            (food_creator())
            (food_searcher())
        }))),
        None => Ok(redirect("/")),
    }
}

#[post("/search_food")]
async fn search_food(
    form: web::Form<SearchData>,
    data: web::Data<AppState<'static>>,
    session: Session,
) -> AwResult<Markup> {
    let matching_foods = web::block(move || {
        let mut conn = data.db.get().expect("Couldnt get db conn from pool");
        use track_notes::schema::foods::dsl;

        dsl::foods
            .filter(foods::name.like(format!("{}%", form.search_name)))
            .load::<Food>(&mut conn)
    })
    .await?;

    let foods: Vec<Food> = matching_foods.unwrap_or(vec![]);

    Ok(html! {
        @for food in &foods {
            tr class="" {
                td class="py-2" {(food.name)}
                td class="text-right py-2" {(format!("{}", food.calories / 100))}
                td class="text-right py-2" {(format!("{}", food.protein / 100))}
            }
        }
    })
}

#[post("/login")]
async fn login(
    form: web::Form<LoginData>,
    data: web::Data<AppState<'static>>,
    session: Session,
) -> AwResult<HttpResponse> {
    println!("Username: {} Password: {}", form.username, form.password);

    let pw_bytes = form.password.clone().into_bytes();
    let user = web::block(move || {
        let mut conn = data.db.get().expect("Couldnt get db conn from pool");
        use track_notes::schema::users::dsl;
        let x = dsl::users
            .filter(users::username.eq(&form.username))
            .load::<User>(&mut conn);
        match x {
            Ok(list) => list,
            Err(_) => vec![],
        }
    })
    .await?;

    if user.len() > 0 {
        println!("{:?}", user);
        let u: &User = &user[0];
        let parsed_hash = PasswordHash::new(&u.password).unwrap();

        if Argon2::default()
            .verify_password(&pw_bytes, &parsed_hash)
            .is_ok()
        {
            session.insert("authenticated", true).unwrap();
            session.insert("userId", u.id).unwrap();
            return Ok(HttpResponse::Ok()
                .content_type(ContentType::html())
                .insert_header(("HX-Redirect", "/meal_builder"))
                .body(""));
        }
    }

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(sign_in_page(Some("Invalid username or password")).into_string()))
}

#[post("/sign-up")]
async fn sign_up_post(
    form: web::Form<LoginData>,
    data: web::Data<AppState<'static>>,
) -> AwResult<HttpResponse> {
    if form.username.len() < 3 {
        return Ok(markup_to_resp(sign_up_page(Some(
            "Username has to be at least 3 characters long",
        ))));
    }

    let user = web::block(move || {
        let mut conn = data.db.get().expect("Couldnt get db conn from pool");
        use track_notes::schema::users::dsl;

        let rand_str: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let salt = SaltString::from_b64(&rand_str).unwrap();
        let argon2 = Argon2::default();
        let pw_bytes = form.password.clone().into_bytes();
        let pw_hash = argon2.hash_password(&pw_bytes, &salt).unwrap().to_string();

        diesel::insert_into(dsl::users)
            .values(&vec![(
                users::username.eq(&form.username),
                users::password.eq(pw_hash),
            )])
            .execute(&mut conn)
    })
    .await?;

    Ok(match user {
        Ok(_) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .insert_header(("HX-Redirect", "/"))
            .body(""),
        Err(e) => match e {
            diesel::result::Error::DatabaseError(de, _) => match de {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    markup_to_resp(sign_up_page(Some("Username already exists")))
                }
                _ => markup_to_resp(sign_up_page(Some("An database error occured"))),
            },
            _ => markup_to_resp(sign_up_page(Some("An unknown error occured"))),
        },
    })
}

#[post("/create_food")]
async fn create_food(
    form: web::Form<CreateFoodData>,
    data: web::Data<AppState<'static>>,
    session: Session,
) -> AwResult<HttpResponse> {
    if let session_data = SessionData::from_session(&session) {
        let food_id = web::block(move || {
            let mut conn = data.db.get().expect("Couldnt get db conn from pool");
            use track_notes::schema::foods::dsl;

            diesel::insert_into(dsl::foods)
                .values((
                    foods::name.eq(&form.name),
                    foods::calories.eq(form.calories * 100),
                    foods::protein.eq(form.protein * 100),
                ))
                .execute(&mut conn)
        })
        .await?;

        return Ok(match food_id {
            Ok(_) => markup_to_resp(
                html! { p id="status" class="text-green-400 font-bold" { "Success" } },
            ),
            Err(_) => {
                markup_to_resp(html! { p id="status" class="text-red-400 font-bold" { "Failed" } })
            }
        });
    } else {
        return Ok(redirect("/"));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let secret_key = Key::generate();
    let secret_key = Key::from(b"RandomlolsecretRandomlolsecreRandomlolsecreRandomlolsecreRandomlolsecreRandomlolsecrettttt");

    let manager = r2d2::ConnectionManager::<SqliteConnection>::new("diesel_demo.sqlite");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Database url should be a valid path to a SQLite DB file");

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(AppState::new(pool.clone())))
            .service(index)
            .service(notes)
            .service(change_track)
            .service(login)
            .service(sign_up)
            .service(sign_up_post)
            .service(meal_builder)
            .service(create_food)
            .service(search_food)
            .service(Files::new("/", "./assets").prefer_utf8(true))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
