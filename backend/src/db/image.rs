use common::Image;
use diesel::prelude::*;
use super::schema::images::dsl::*;
use super::DbConnection;
use diesel::result::Error;

pub(crate) type ImageId = i32;

pub(crate) fn get_image(mut conn: DbConnection, image_id: ImageId) -> Result<Image, Error> {
    images.find(image_id).first(&mut conn)
} 

