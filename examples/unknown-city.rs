fn main() {
    let api = france_api_adresse::client::BAN::new();
    let search = api.geocode("1914 Route du marais".to_string());

    let search = search.postcode("74999");
    let search = search.city("Blifablette".to_string());

    // Get the results
    let result = search.execute_blocking();

    println!("Search results:");

    if let Ok(result) = result {
        println!("Results: {:?}", result);
        for result in result.features {
            println!("Address: {}", result.properties.label);
            println!("Coords: {:?}", result.geometry.coordinates);
        }
    } else {
        println!("Error: {:?}", result.err());
    }
}
