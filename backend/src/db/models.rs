use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use base64::{encode, decode};
use chrono::NaiveDateTime;
use crate::db::schema::*;

// Estrutura que representa a tabela `users`
#[derive(Queryable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub nickname: String,
    pub public_key: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

// Estrutura para inserção de novos usuários
#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub nickname: String,
    pub public_key: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

// Estrutura que representa a tabela `file`
#[derive(Queryable, Serialize)]
#[diesel(table_name = file)]
pub struct File {
    pub id: i32,
    pub owner_id: i32,
    pub name: String,
    pub path: String,
    pub key: String, // AES key codificada em base64
    pub created_at: Option<NaiveDateTime>,
}

// Estrutura para inserção de novos arquivos
#[derive(Insertable, Deserialize)]
#[diesel(table_name = file)]
pub struct NewFile {
    pub owner_id: i32,
    pub name: String,
    pub path: String,
    pub key: String, // AES key codificada em base64
    pub created_at: Option<NaiveDateTime>,
}

// Estrutura que representa a tabela `file_share`
#[derive(Queryable, Serialize)]
#[diesel(table_name = file_share)]
pub struct FileShare {
    pub id: i32,
    pub file_id: i32,
    pub user_id: i32,
    pub key: String, // AES key codificada em base64
    pub created_at: Option<NaiveDateTime>,
}

// Estrutura para inserção de compartilhamento de arquivos
#[derive(Insertable, Deserialize)]
#[diesel(table_name = file_share)]
pub struct NewFileShare {
    pub file_id: i32,
    pub user_id: i32,
    pub key: String, // AES key codificada em base64
    pub created_at: Option<NaiveDateTime>,
}

// Implementação de funções auxiliares para File
impl File {
    // Decodifica a chave AES de base64
    pub fn decode_aes_key(&self) -> Result<Vec<u8>, base64::DecodeError> {
        decode(&self.key)
    }
}

impl NewFile {
    // Codifica a chave AES para base64 antes de salvar
    pub fn encode_aes_key(key: Vec<u8>) -> String {
        encode(key)
    }
}

// Estrutura para atualização de usuários
#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub nickname: Option<&'a str>, 
    pub password: Option<&'a str>,
    pub public_key: Option<&'a str>,
}
