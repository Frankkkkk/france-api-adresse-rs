fn main() {
    let api = france_api_adresse::client::BAN::new();
    let search = api.geocode("Route du marais".to_string());

    // Narrow the results to post code 74380
    let search = search.postcode("74380");

    // Narrow the results to city "Cranves-Sales"
    let search = search.city("Cranves-Sales".to_string());

    // Get the results
    let result = search.execute_blocking().unwrap();
    for result in result.features {
        println!("Address: {}", result.properties.label);
        println!("Coords: {:?}", result.geometry.coordinates);
    }
}
