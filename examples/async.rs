#[tokio::main]
pub async fn main() {
    let result = france_api_adresse::async_api::get_address_info("200 Chemin de puy petit")
        .await
        .unwrap();

    println!("Address Info: {:?}", result);
}
