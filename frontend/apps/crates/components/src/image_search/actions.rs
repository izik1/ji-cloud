use std::rc::Rc;
use dominator::{clone};
use shared::{api::{ApiEndpoint, endpoints}, domain::{image::*, meta::*}, error::{EmptyError, MetadataNotFound}};
use utils::fetch::{api_upload_file, api_with_auth};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::File;
use super::state::{BACKGROUND_NAME, State};

pub async fn get_styles() -> Vec<Style> {
    let res = api_with_auth::<MetadataResponse, (), ()>(
        &endpoints::meta::Get::PATH,
        endpoints::meta::Get::METHOD,
        None
    ).await;
    res.unwrap_throw().styles
}

pub fn get_background_id(styles: &Vec<Style>) -> StyleId {
    styles
        .iter()
        .find(|s| s.display_name == BACKGROUND_NAME)
        .unwrap_throw()
        .id
        .clone()
}

pub fn search(state: Rc<State>) {
    let search_query = ImageSearchQuery {
        q: state.query.lock_ref().clone(),
        page: state.page.lock_ref().clone(),
        styles: state.selected_styles
            .borrow()
            .iter()
            .map(|style_id| style_id.clone())
            .collect(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new(),
        is_premium: None,
        is_published: None,
        kind: Some(ImageKind::Sticker),
    };
    state.loader.load(clone!(state => async move {
        let res = api_with_auth::<ImageSearchResponse, EmptyError, _>(
            &endpoints::image::Search::PATH,
            endpoints::image::Search::METHOD,
            Some(search_query)
        ).await;
        match res {
            Ok(res) => {
                state.image_list
                    .lock_mut()
                    .replace_cloned(res.images.iter().map(|ir| ir.metadata.clone())
                    .collect());
            },
            Err(e) => {
                log::error!("{:#?}", e);
            }
        }
    }));
}

pub async fn upload_file(state: Rc<State>, file: File) {
    let req = ImageCreateRequest {
        name: "".to_string(),
        description: "".to_string(),
        is_premium: false,
        publish_at: None,
        styles: Vec::new(),
        age_ranges: Vec::new(),
        affiliations: Vec::new(),
        categories: Vec::new(),
        kind: ImageKind::Sticker,
    };

    match api_with_auth::<CreateResponse, MetadataNotFound, _>(endpoints::image::Create::PATH, endpoints::image::Create::METHOD, Some(req)).await {
        Ok(resp) => {
            let CreateResponse { id } = resp;

            let path = endpoints::image::Upload::PATH.replace("{id}", &id.0.to_string());
            match api_upload_file(&path, &file, endpoints::image::Upload::METHOD).await {
                Ok(_) => {
                    state.options.value.set(Some(resp.id));
                },
                Err(_) => {
                    log::error!("error uploading!");
                }
            }
        },
        Err(_) => {
            log::error!("error creating image db!")
        }
    }
}