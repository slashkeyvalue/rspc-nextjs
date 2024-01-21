use rspc::RouterBuilder;
use super::{Ctx, PublicRouter};

use lazy_static::lazy_static;
use regex::Regex;

use serde::{Deserialize, Serialize};
use specta::Type;
use validator::Validate;

use uuid::Uuid;

use axum::http::HeaderMap;
use axum::http::header::CONTENT_TYPE;

lazy_static! {
    static ref REGEX_FILE_CONTENT_TYPE: Regex = Regex::new(r"^[a-z]{2}$").unwrap();
}

pub(crate) fn public_route() -> RouterBuilder<Ctx>
{
    PublicRouter::new()
        .mutation("upload", |t|
        {

            #[derive(Deserialize, Type, Debug, Validate)]
            pub struct FileUploadInput
            {
                pub file_name: String,

                #[validate(regex = "REGEX_FILE_CONTENT_TYPE")]
                pub content_type: String,
            }

            #[derive(Serialize, Deserialize, Type)]
            pub struct FileUpload
            {
                pub signed_url: String,
            }

            t(| ctx, input: FileUploadInput | async move
            {
                let file_key = format!("{}-{}", Uuid::new_v4(), input.file_name);

                let mut custom_headers = HeaderMap::with_capacity(1);

                custom_headers.insert(CONTENT_TYPE, input.content_type.parse().unwrap());

                let signed_put_url = ctx.bucket.presign_put( file_key, 600, Some( custom_headers ) ).await.unwrap();

                Ok(
                    FileUpload {
                        signed_url: signed_put_url,
                    }
                )
            })
        })
}
