use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::db::models;

pub fn create_user( conn: &mut PgConnection, nickname: String, password: String, public_key: String ) -> models::User {
    use crate::db::schema::users;

    use chrono::Local;
    let now = Local::now().naive_local();

    let new_user = models::NewUser {
        nickname,  
        password,  
        public_key,
        created_at: Some(now),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn) 
        .expect("Error saving new user")
}

pub fn update_user( conn: &mut PgConnection, user_id: i32, nickname: String, password: String, public_key: String ) -> models::User {
    use crate::db::schema::users;

    let updated_user = models::UpdateUser {
        nickname: if nickname.is_empty() { None } else { Some(&nickname) },
        password: if password.is_empty() { None } else { Some(&password) },
        public_key: if public_key.is_empty() { None } else { Some(&public_key) },
    };

    diesel::update(users::table.find(user_id))
        .set(updated_user)
        .get_result(conn)
        .expect(&format!("Unable to find user {}", user_id))
}

pub fn delete_user( conn: &mut PgConnection, user_id: i32 ) -> usize {
    use crate::db::schema::users::dsl::*;

    diesel::delete(users.find(user_id))
        .execute(conn)
        .expect("Error deleting user")
}

pub fn get_user_by_id( conn: &mut PgConnection, user_id: i32 ) -> models::User {
    use crate::db::schema::users::dsl::*;

    users.find(user_id)
        .first(conn)
        .expect("Error loading user")
}

pub fn get_user_by_nickname( conn: &mut PgConnection, user_nickname: String ) -> models::User {
    use crate::db::schema::users::dsl::*;

    users.filter(nickname.eq(user_nickname))
        .first(conn)
        .expect("Error loading user")
}

pub fn get_user_by_public_key( conn: &mut PgConnection, user_public_key: String ) -> models::User {
    use crate::db::schema::users::dsl::*;

    users.filter(public_key.eq(user_public_key))
        .first(conn)
        .expect("Error loading user")
}
