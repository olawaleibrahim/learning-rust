pub fn country_code_matching() {
    let country_code = 1000;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };
    println!("The country with code {} is {}", country_code, country);
}