use crate::{
    models::user::UserModel, repositories::user_repository, schemas::user::UpdatePhotoUserSchema,
    utils::config::Config,
};
use actix_multipart::Multipart;
use futures_util::{StreamExt, TryStreamExt};
use sqlx::{mysql::MySqlQueryResult, MySqlPool};
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct UserService {
    pool: MySqlPool,
}

impl UserService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_user(
        &self,
        user_id: Option<&str>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<UserModel>, sqlx::Error> {
        let user = user_repository::get_user(user_id, name, email, self.pool.clone()).await?;
        Ok(user)
    }

    pub async fn update_photo(
        &self,
        photo_id: Option<&str>,
        user_id: Option<&str>,
        mut payload: Multipart,
    ) -> Result<MySqlQueryResult, String> {
        let mut form_data = UpdatePhotoUserSchema { file: None };
        let config = Config::init().to_owned();
        let mut saved_name = "default.png".to_owned();

        // Save file to STORAGE_DIR
        while let Ok(Some(mut field)) = payload.try_next().await {
            let mut buffer = Vec::new();

            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };
                buffer.extend_from_slice(&data);
            }

            if field.name() == "file" {
                form_data.file = Some(buffer.clone());

                match field.content_disposition().get_filename() {
                    Some(filename) => {
                        if let Some(extension) = filename.rfind(".") {
                            let extension = &filename[extension..];

                            saved_name = match photo_id {
                                Some(id) => format!("{}{}", id, extension),
                                None => String::from("default.png"),
                            };

                            let destination: String =
                                format!("{}{}", config.storage_dir, saved_name,);

                            let mut file = match File::create(destination) {
                                Ok(file) => file,
                                Err(e) => return Err(e.to_string()),
                            };

                            match file.write_all(&form_data.file.unwrap()) {
                                Ok(_) => {}
                                Err(e) => return Err(e.to_string()),
                            };
                        };
                    }
                    None => (),
                };
            }
        }

        let query_result =
            user_repository::update_user(user_id.unwrap(), Some(&saved_name), self.pool.clone())
                .await;

        Ok(query_result?)
    }
}
