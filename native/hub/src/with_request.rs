//! This module runs the corresponding function
//! when a `RustRequest` was received from Dart
//! and returns a `RustResponse`.

use prost::Message;

use crate::bridge::{RustRequestUnique, RustResponse, RustResponseUnique};
use crate::messages;
use crate::sample_functions;

pub async fn handle_request(request_unique: RustRequestUnique) -> RustResponseUnique {
    // Get the request data from Dart.
    let rust_request = request_unique.request;
    let interaction_id = request_unique.id;

    // Run the function that handles the Rust resource.
    let rust_resource = rust_request.resource;
    let rust_response = match rust_resource {
        messages::sample_folder::sample_resource::ID => {
            sample_functions::handle_sample_resource(rust_request).await
        }
        messages::sample_folder::deeper_folder::deeper_resource::ID => {
            sample_functions::handle_deeper_resource(rust_request).await
        }
        messages::bg::ID => {
            let bytes = rust_request.message.unwrap();
            let req = messages::bg::GenRequest::decode(bytes.as_ref()).unwrap();
            let resp = handel_gen(req).await;
            RustResponse {
                successful: true,
                message: Some(resp.encode_to_vec()),
                blob: None,
            }
        }
        _ => RustResponse::default(),
    };

    // Return the response to Dart.
    RustResponseUnique {
        id: interaction_id,
        response: rust_response,
    }
}

async fn handel_gen(req: messages::bg::GenRequest) -> messages::bg::GenResponse {
    let cfg = bmps::Config {
        size: bmps::config::Size {
            width: req.width,
            height: req.height,
            blur_radius: req.blur_radius,
            shadow: req.shadow,
            round_radius: req.round_radius,
            padding: req.padding,
        },
        source_file: req.source,
        dest_file: req.dest,
        font: None,
    };
    crate::debug_print!("params {cfg:?}");
    let res = tokio_with_wasm::tokio::task::spawn_blocking(move || bmps::go(cfg)).await;
    crate::debug_print!("result {res:?}");
    match res {
        Ok(Ok(_)) => messages::bg::GenResponse::default(),
        Ok(Err(e)) => messages::bg::GenResponse {
            code: 1,
            msg: format!("{e:?}"),
        },
        Err(e) => messages::bg::GenResponse {
            code: 2,
            msg: format!("{e:?}"),
        },
    }
}
