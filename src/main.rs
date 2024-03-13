extern crate openssl;
extern crate rui;

use std::fmt::Alignment;

use openssl::{asn1::Asn1Time, x509::{X509NameBuilder, X509, X509Req, X509ReqBuilder}, rsa::Rsa, pkey::PKey};
use rui::*;

fn main() {
    let lorem = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    rui(
        vstack(
            (state(
            move || lorem.to_string(),
            |state, _| text_editor(state).padding(Auto),
        )
        .background(
            rectangle()
                .color(BUTTON_BACKGROUND_COLOR)
                .corner_radius(5.0),
        )
        .padding(Auto))
        .window_title("Certificate Generator")
    ));
}


fn generate_csr() {
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let mut name_builder = X509NameBuilder::new().unwrap();
    name_builder.append_entry_by_text("C", "US").unwrap();
    name_builder.append_entry_by_text("ST", "California").unwrap();
    name_builder.append_entry_by_text("L", "San Francisco").unwrap();
    name_builder.append_entry_by_text("O", "My Organization").unwrap();
    name_builder.append_entry_by_text("CN", "my.example.com").unwrap();
    let name = name_builder.build();

    let mut req_builder = X509ReqBuilder::new().unwrap();
    req_builder.set_version(0).unwrap();
    req_builder.set_subject_name(&name).unwrap();
    req_builder.set_pubkey(&pkey).unwrap();
    req_builder.sign(&pkey, openssl::hash::MessageDigest::sha256()).unwrap();

    let req = req_builder.build();

    println!("{:?}", req.to_text().unwrap());
}

