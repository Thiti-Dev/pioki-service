use actix_web::{dev::ServiceRequest, HttpMessage};

#[derive(Clone,Debug)]
pub struct PIOKIIdentifierData {
    pub id: String,
}

///@Usage
/// ```
///create_user(_: HttpRequest,identifier_data: Option<ReqData<PIOKIIdentifierData>>)
///```
pub fn identifier_extractor(req: &ServiceRequest){
    let headers = req.headers();
    let identifier_header = headers.get("pioki-identifier");

    // only attach the value if found
    if let Some(id_header) = identifier_header {
        println!("requested: {} has been attached with PIOKI's identifier", req.path());
        req.extensions_mut().insert(PIOKIIdentifierData{id: id_header.to_str().unwrap().to_string()});
    }
}