use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::form::Form;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct LoginForm {
    nickname: String,
    password: String,
}

// Função para efetuar login
#[post("/login", data = "<login_form>")]
fn login(login_form: Form<LoginForm>, cookies: &CookieJar<'_>) -> Redirect {
    let nickname = &login_form.nickname;
    let password = &login_form.password;

    // Simule a busca no banco de dados
    let user = find_user_by_nickname(nickname); // Implementar essa função

    if let Some(user) = user {
        if verify_password(&user.password, password) {
            // Salva o identificador do usuário nos cookies
            cookies.add_private(Cookie::new("user_id", user.id.to_string()));
            return Redirect::to("/dashboard");
        }
    }

    Redirect::to("/login?error=invalid_credentials")
}

// Função para logout
#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}

// Middleware para proteger rotas
#[get("/dashboard")]
fn dashboard(cookies: &CookieJar<'_>) -> &'static str {
    if let Some(cookie) = cookies.get_private("user_id") {
        // Aqui você pode usar `cookie.value()` para buscar mais informações
        "Bem-vindo ao seu dashboard!"
    } else {
        "Por favor, faça login primeiro."
    }
}

// Simulação de busca no banco
fn find_user_by_nickname(nickname: &str) -> Option<User> {
    // Substituir pela busca real no banco de dados
    Some(User {
        id: 1,
        nickname: nickname.to_string(),
        password: hash_password("senha123"), // Simule o hash da senha
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![login, logout, dashboard])
}
