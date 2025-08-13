#[test]
fn geocode_test() {
    let api = france_api_adresse::client::BAN::new();
    let search = api.geocode("Route du marais".to_string());

    let results = search.execute_blocking().unwrap();

    println!("Results: {:?}", results);

    assert!(results.features.len() > 2);
}

#[test]
fn geocode_filter_results() {
    let api = france_api_adresse::client::BAN::new();
    let search = api.geocode("Route des marais".to_string());

    // Narrow the results to post code 74380
    let search = search.postcode("74380");

    // Narrow the results to city "Cranves-Sales"
    let search = search.city("Cranves-Sales".to_string());

    let results = search.execute_blocking().unwrap();

    assert!(results.features[0].properties.r#type == "street");
    assert!(results.features[0].properties.postcode == "74380");
}

#[test]
fn geocode_order_by_latlon() {
    let api = france_api_adresse::client::BAN::new();
    let search = api.geocode("Route des marais".to_string());

    let search = search.around_lat_lon(46.2, 6.3);

    // Order by latitude and longitude
    let results = search.limit_results(1).execute_blocking().unwrap();

    println!("Results: {:#?}", results);

    assert!(results.features[0].properties.postcode == "74380");
}
