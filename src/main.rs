use actix_web::web::Bytes;
use actix_web::{get, App, HttpServer};
use actix_web::{HttpResponse, Responder};
use opencv::core::BorderTypes;
use opencv::core::Vector as OpencvVec;
use opencv::imgcodecs::{imdecode, imencode};
use opencv::imgproc::gaussian_blur;
use opencv::prelude::Mat;

pub fn process_image(image_data: Mat) -> Result<Mat, opencv::Error> {
    let mut output_buffer = Mat::default();

    let kernel_size = opencv::core::Size::new(5, 5);
    let sigma_x = 0.0;
    let sigma_y = 0.0;

    gaussian_blur(
        &image_data,
        &mut output_buffer,
        kernel_size,
        sigma_x,
        sigma_y,
        BorderTypes::BORDER_CONSTANT as i32,
    )?;

    Ok(output_buffer)
}

#[get("/applyblur")]
pub async fn apply_blur(query: Bytes) -> impl Responder {
    let input = opencv::types::VectorOfu8::from_iter(query);
    let decoded_mat = imdecode(&input, opencv::imgcodecs::IMREAD_ANYCOLOR).unwrap();
    match process_image(decoded_mat) {
        Ok(img) => {
            let mut buffer = OpencvVec::new();
            let encode_params = opencv::types::VectorOfi32::from_iter(vec![
                opencv::imgcodecs::IMWRITE_JPEG_QUALITY,
                100,
            ]);

            imencode(".jpg", &img, &mut buffer, &encode_params).unwrap();
            std::fs::write("response.jpg", &buffer).unwrap();

            HttpResponse::Ok()
                .content_type("image/jpg")
                .body(Bytes::from_iter(buffer))
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("LD_LIBRARY_PATH", "lib/");
    env_logger::init();
    log::info!("Starting server on localhost:8080");
    HttpServer::new(|| App::new().service(apply_blur))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
