use france_api_adresse::types::FilterType;

fn main() {
    let api = france_api_adresse::client::BAN::new();

    let add = api
        .reverse(46.209011, 6.303156)
        .filter_type(FilterType::Street);

    let result = add.execute_blocking().unwrap();

    for feature in result.features {
        println!("Address: {}", feature.properties.label);
        println!("Coords: {:?}", feature.geometry.coordinates);
    }
}
