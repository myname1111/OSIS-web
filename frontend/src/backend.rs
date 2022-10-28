use reqwasm::http::Request;
use common::*;

// TODO: delete get_* for better error handling

pub async fn get_member(id: u32) -> Member {
    try_get_member(id).await.expect("Cannot get member")
}

pub async fn try_get_member(id: u32) -> Option<Member> {
    log::info!("Getting member with  id {}", id);
    let url = format!("http://localhost/api/member/{}", id);
    let member: Option<Member> = Request::get(url.as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    log::info!("{}", if member.is_some() { "Success!" } else { "No member found" });
    log::debug!("{:?}", member);
    member
}

pub async fn get_image(id: u32) -> Image {
    try_get_image(id).await.expect("Unable to get image")
}

pub async fn try_get_image(id: u32) -> Option<Image> {
    log::info!("Getting img with id {}", id);
    let url = format!("http://localhost/api/image/{}", id);
    
    let image: Option<Image> = Request::get(url.as_str())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    
    log::info!("{}", if image.is_some() { "Success!" } else { "No image found" });
    log::debug!("{:?}", image);
    image
}
