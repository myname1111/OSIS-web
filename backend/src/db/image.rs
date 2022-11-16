use super::schema::images::dsl::*;
use super::DbConnection;
use common::Image;
use diesel::prelude::*;
use diesel::result::Error;

pub type ImageId = i32;

pub(crate) fn get_image(mut conn: DbConnection, image_id: ImageId) -> Result<Image, Error> {
    images.find(image_id).first(&mut conn)
}
