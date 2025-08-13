use france_api_adresse;
fn main() {
    let result =
        france_api_adresse::blocking_api::get_address_info("200 Chemin de puy petit").unwrap();

    println!("Address Info: {:?}", result);
}
