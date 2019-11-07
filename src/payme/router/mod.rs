extern crate mount;
extern crate staticfile;

use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;

use payme::handler;

fn make_index_router() -> Router {
    router!(
        index: get "/" => handler::handle_index_request
    )
}

fn make_invoice_router() -> Router {
    router!(
        invoice: post "/" => handler::handle_invoice_request
    )
}

fn make_receipt_router() -> Router {
    router!(
        receipt: get "/:invoice_id" => handler::handle_receipt_request
    )
}

fn make_unsubscribe_router() -> Router {
    router!(
        unsubscribe: get "/" => handler::handle_unsubscribe_request
    )
}

pub fn make_mount() -> Mount {
    let mut mount = Mount::new();
    mount.mount("/", make_index_router());
    mount.mount("/invoice/", make_invoice_router());
    mount.mount("/receipt/", make_receipt_router());
    mount.mount("/unsubscribe/", make_unsubscribe_router());
    mount.mount(
        "/js/",
        Static::new(Path::new("web-app/resources/public/js/")),
    );
    mount.mount(
        "/css/",
        Static::new(Path::new("web-app/resources/public/css/")),
    );
    mount.mount(
        "/scss/",
        Static::new(Path::new("web-app/resources/public/scss/")),
    );
    mount
}
