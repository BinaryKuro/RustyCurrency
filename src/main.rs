use axum::{
    extract::Query,
    response::Json,
    routing::get,
    Router,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CountryQuery {
    based: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountryInfo {
    country: String,
    flag: String,
    #[serde(rename = "currencyCode")]
    currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CountryResponse {
    results: Vec<CountryInfo>,
}

// Global country data initialized once - All 195 UN-recognized countries
static COUNTRY_DATA: Lazy<HashMap<String, (String, String)>> = Lazy::new(|| {
    let mut data = HashMap::new();
    
    // Format: (flag emoji, currency code)
    // A
    data.insert("afghanistan".to_string(), ("🇦🇫".to_string(), "AFN".to_string()));
    data.insert("albania".to_string(), ("🇦🇱".to_string(), "ALL".to_string()));
    data.insert("algeria".to_string(), ("🇩🇿".to_string(), "DZD".to_string()));
    data.insert("andorra".to_string(), ("🇦🇩".to_string(), "EUR".to_string()));
    data.insert("angola".to_string(), ("🇦🇴".to_string(), "AOA".to_string()));
    data.insert("antigua and barbuda".to_string(), ("🇦🇬".to_string(), "XCD".to_string()));
    data.insert("antigua".to_string(), ("🇦🇬".to_string(), "XCD".to_string()));
    data.insert("argentina".to_string(), ("🇦🇷".to_string(), "ARS".to_string()));
    data.insert("armenia".to_string(), ("🇦🇲".to_string(), "AMD".to_string()));
    data.insert("australia".to_string(), ("🇦🇺".to_string(), "AUD".to_string()));
    data.insert("austria".to_string(), ("🇦🇹".to_string(), "EUR".to_string()));
    data.insert("azerbaijan".to_string(), ("🇦🇿".to_string(), "AZN".to_string()));
    
    // B
    data.insert("bahamas".to_string(), ("🇧🇸".to_string(), "BSD".to_string()));
    data.insert("the bahamas".to_string(), ("🇧🇸".to_string(), "BSD".to_string()));
    data.insert("bahrain".to_string(), ("🇧🇭".to_string(), "BHD".to_string()));
    data.insert("bangladesh".to_string(), ("🇧🇩".to_string(), "BDT".to_string()));
    data.insert("barbados".to_string(), ("🇧🇧".to_string(), "BBD".to_string()));
    data.insert("belarus".to_string(), ("🇧🇾".to_string(), "BYN".to_string()));
    data.insert("belgium".to_string(), ("🇧🇪".to_string(), "EUR".to_string()));
    data.insert("belize".to_string(), ("🇧🇿".to_string(), "BZD".to_string()));
    data.insert("benin".to_string(), ("🇧🇯".to_string(), "XOF".to_string()));
    data.insert("bhutan".to_string(), ("🇧🇹".to_string(), "BTN".to_string()));
    data.insert("bolivia".to_string(), ("🇧🇴".to_string(), "BOB".to_string()));
    data.insert("bosnia and herzegovina".to_string(), ("🇧🇦".to_string(), "BAM".to_string()));
    data.insert("bosnia".to_string(), ("🇧🇦".to_string(), "BAM".to_string()));
    data.insert("botswana".to_string(), ("🇧🇼".to_string(), "BWP".to_string()));
    data.insert("brazil".to_string(), ("🇧🇷".to_string(), "BRL".to_string()));
    data.insert("brunei".to_string(), ("🇧🇳".to_string(), "BND".to_string()));
    data.insert("bulgaria".to_string(), ("🇧🇬".to_string(), "BGN".to_string()));
    data.insert("burkina faso".to_string(), ("🇧🇫".to_string(), "XOF".to_string()));
    data.insert("burundi".to_string(), ("🇧🇮".to_string(), "BIF".to_string()));
    
    // C
    data.insert("cabo verde".to_string(), ("🇨🇻".to_string(), "CVE".to_string()));
    data.insert("cape verde".to_string(), ("🇨🇻".to_string(), "CVE".to_string()));
    data.insert("cambodia".to_string(), ("🇰🇭".to_string(), "KHR".to_string()));
    data.insert("cameroon".to_string(), ("🇨🇲".to_string(), "XAF".to_string()));
    data.insert("canada".to_string(), ("🇨🇦".to_string(), "CAD".to_string()));
    data.insert("central african republic".to_string(), ("🇨🇫".to_string(), "XAF".to_string()));
    data.insert("chad".to_string(), ("🇹🇩".to_string(), "XAF".to_string()));
    data.insert("chile".to_string(), ("🇨🇱".to_string(), "CLP".to_string()));
    data.insert("china".to_string(), ("🇨🇳".to_string(), "CNY".to_string()));
    data.insert("colombia".to_string(), ("🇨🇴".to_string(), "COP".to_string()));
    data.insert("comoros".to_string(), ("🇰🇲".to_string(), "KMF".to_string()));
    data.insert("congo".to_string(), ("🇨🇬".to_string(), "XAF".to_string()));
    data.insert("republic of the congo".to_string(), ("🇨🇬".to_string(), "XAF".to_string()));
    data.insert("democratic republic of the congo".to_string(), ("🇨🇩".to_string(), "CDF".to_string()));
    data.insert("dr congo".to_string(), ("🇨🇩".to_string(), "CDF".to_string()));
    data.insert("drc".to_string(), ("🇨🇩".to_string(), "CDF".to_string()));
    data.insert("costa rica".to_string(), ("🇨🇷".to_string(), "CRC".to_string()));
    data.insert("croatia".to_string(), ("🇭🇷".to_string(), "EUR".to_string()));
    data.insert("cuba".to_string(), ("🇨🇺".to_string(), "CUP".to_string()));
    data.insert("cyprus".to_string(), ("🇨🇾".to_string(), "EUR".to_string()));
    data.insert("czech republic".to_string(), ("🇨🇿".to_string(), "CZK".to_string()));
    data.insert("czechia".to_string(), ("🇨🇿".to_string(), "CZK".to_string()));
    data.insert("côte d'ivoire".to_string(), ("🇨🇮".to_string(), "XOF".to_string()));
    data.insert("ivory coast".to_string(), ("🇨🇮".to_string(), "XOF".to_string()));
    
    // D
    data.insert("denmark".to_string(), ("🇩🇰".to_string(), "DKK".to_string()));
    data.insert("djibouti".to_string(), ("🇩🇯".to_string(), "DJF".to_string()));
    data.insert("dominica".to_string(), ("🇩🇲".to_string(), "XCD".to_string()));
    data.insert("dominican republic".to_string(), ("🇩🇴".to_string(), "DOP".to_string()));
    
    // E
    data.insert("ecuador".to_string(), ("🇪🇨".to_string(), "USD".to_string()));
    data.insert("egypt".to_string(), ("🇪🇬".to_string(), "EGP".to_string()));
    data.insert("el salvador".to_string(), ("🇸🇻".to_string(), "USD".to_string()));
    data.insert("equatorial guinea".to_string(), ("🇬🇶".to_string(), "XAF".to_string()));
    data.insert("eritrea".to_string(), ("🇪🇷".to_string(), "ERN".to_string()));
    data.insert("estonia".to_string(), ("🇪🇪".to_string(), "EUR".to_string()));
    data.insert("eswatini".to_string(), ("🇸🇿".to_string(), "SZL".to_string()));
    data.insert("swaziland".to_string(), ("🇸🇿".to_string(), "SZL".to_string()));
    data.insert("ethiopia".to_string(), ("🇪🇹".to_string(), "ETB".to_string()));
    
    // F
    data.insert("fiji".to_string(), ("🇫🇯".to_string(), "FJD".to_string()));
    data.insert("finland".to_string(), ("🇫🇮".to_string(), "EUR".to_string()));
    data.insert("france".to_string(), ("🇫🇷".to_string(), "EUR".to_string()));
    
    // G
    data.insert("gabon".to_string(), ("🇬🇦".to_string(), "XAF".to_string()));
    data.insert("gambia".to_string(), ("🇬🇲".to_string(), "GMD".to_string()));
    data.insert("the gambia".to_string(), ("🇬🇲".to_string(), "GMD".to_string()));
    data.insert("georgia".to_string(), ("🇬🇪".to_string(), "GEL".to_string()));
    data.insert("germany".to_string(), ("🇩🇪".to_string(), "EUR".to_string()));
    data.insert("ghana".to_string(), ("🇬🇭".to_string(), "GHS".to_string()));
    data.insert("greece".to_string(), ("🇬🇷".to_string(), "EUR".to_string()));
    data.insert("grenada".to_string(), ("🇬🇩".to_string(), "XCD".to_string()));
    data.insert("guatemala".to_string(), ("🇬🇹".to_string(), "GTQ".to_string()));
    data.insert("guinea".to_string(), ("🇬🇳".to_string(), "GNF".to_string()));
    data.insert("guinea-bissau".to_string(), ("🇬🇼".to_string(), "XOF".to_string()));
    data.insert("guyana".to_string(), ("🇬🇾".to_string(), "GYD".to_string()));
    
    // H
    data.insert("haiti".to_string(), ("🇭🇹".to_string(), "HTG".to_string()));
    data.insert("honduras".to_string(), ("🇭🇳".to_string(), "HNL".to_string()));
    data.insert("hungary".to_string(), ("🇭🇺".to_string(), "HUF".to_string()));
    
    // I
    data.insert("iceland".to_string(), ("🇮🇸".to_string(), "ISK".to_string()));
    data.insert("india".to_string(), ("🇮🇳".to_string(), "INR".to_string()));
    data.insert("indonesia".to_string(), ("🇮🇩".to_string(), "IDR".to_string()));
    data.insert("iran".to_string(), ("🇮🇷".to_string(), "IRR".to_string()));
    data.insert("iraq".to_string(), ("🇮🇶".to_string(), "IQD".to_string()));
    data.insert("ireland".to_string(), ("🇮🇪".to_string(), "EUR".to_string()));
    data.insert("israel".to_string(), ("🇮🇱".to_string(), "ILS".to_string()));
    data.insert("italy".to_string(), ("🇮🇹".to_string(), "EUR".to_string()));
    
    // J
    data.insert("jamaica".to_string(), ("🇯🇲".to_string(), "JMD".to_string()));
    data.insert("japan".to_string(), ("🇯🇵".to_string(), "JPY".to_string()));
    data.insert("jordan".to_string(), ("🇯🇴".to_string(), "JOD".to_string()));
    
    // K
    data.insert("kazakhstan".to_string(), ("🇰🇿".to_string(), "KZT".to_string()));
    data.insert("kenya".to_string(), ("🇰🇪".to_string(), "KES".to_string()));
    data.insert("kiribati".to_string(), ("🇰🇮".to_string(), "AUD".to_string()));
    data.insert("north korea".to_string(), ("🇰🇵".to_string(), "KPW".to_string()));
    data.insert("south korea".to_string(), ("🇰🇷".to_string(), "KRW".to_string()));
    data.insert("korea".to_string(), ("🇰🇷".to_string(), "KRW".to_string()));
    data.insert("kuwait".to_string(), ("🇰🇼".to_string(), "KWD".to_string()));
    data.insert("kyrgyzstan".to_string(), ("🇰🇬".to_string(), "KGS".to_string()));
    
    // L
    data.insert("laos".to_string(), ("🇱🇦".to_string(), "LAK".to_string()));
    data.insert("latvia".to_string(), ("🇱🇻".to_string(), "EUR".to_string()));
    data.insert("lebanon".to_string(), ("🇱🇧".to_string(), "LBP".to_string()));
    data.insert("lesotho".to_string(), ("🇱🇸".to_string(), "LSL".to_string()));
    data.insert("liberia".to_string(), ("🇱🇷".to_string(), "LRD".to_string()));
    data.insert("libya".to_string(), ("🇱🇾".to_string(), "LYD".to_string()));
    data.insert("liechtenstein".to_string(), ("🇱🇮".to_string(), "CHF".to_string()));
    data.insert("lithuania".to_string(), ("🇱🇹".to_string(), "EUR".to_string()));
    data.insert("luxembourg".to_string(), ("🇱🇺".to_string(), "EUR".to_string()));
    
    // M
    data.insert("madagascar".to_string(), ("🇲🇬".to_string(), "MGA".to_string()));
    data.insert("malawi".to_string(), ("🇲🇼".to_string(), "MWK".to_string()));
    data.insert("malaysia".to_string(), ("🇲🇾".to_string(), "MYR".to_string()));
    data.insert("maldives".to_string(), ("🇲🇻".to_string(), "MVR".to_string()));
    data.insert("mali".to_string(), ("🇲🇱".to_string(), "XOF".to_string()));
    data.insert("malta".to_string(), ("🇲🇹".to_string(), "EUR".to_string()));
    data.insert("marshall islands".to_string(), ("🇲🇭".to_string(), "USD".to_string()));
    data.insert("mauritania".to_string(), ("🇲🇷".to_string(), "MRU".to_string()));
    data.insert("mauritius".to_string(), ("🇲🇺".to_string(), "MUR".to_string()));
    data.insert("mexico".to_string(), ("🇲🇽".to_string(), "MXN".to_string()));
    data.insert("micronesia".to_string(), ("🇫🇲".to_string(), "USD".to_string()));
    data.insert("moldova".to_string(), ("🇲🇩".to_string(), "MDL".to_string()));
    data.insert("monaco".to_string(), ("🇲🇨".to_string(), "EUR".to_string()));
    data.insert("mongolia".to_string(), ("🇲🇳".to_string(), "MNT".to_string()));
    data.insert("montenegro".to_string(), ("🇲🇪".to_string(), "EUR".to_string()));
    data.insert("morocco".to_string(), ("🇲🇦".to_string(), "MAD".to_string()));
    data.insert("mozambique".to_string(), ("🇲🇿".to_string(), "MZN".to_string()));
    data.insert("myanmar".to_string(), ("🇲🇲".to_string(), "MMK".to_string()));
    data.insert("burma".to_string(), ("🇲🇲".to_string(), "MMK".to_string()));
    
    // N
    data.insert("namibia".to_string(), ("🇳🇦".to_string(), "NAD".to_string()));
    data.insert("nauru".to_string(), ("🇳🇷".to_string(), "AUD".to_string()));
    data.insert("nepal".to_string(), ("🇳🇵".to_string(), "NPR".to_string()));
    data.insert("netherlands".to_string(), ("🇳🇱".to_string(), "EUR".to_string()));
    data.insert("new zealand".to_string(), ("🇳🇿".to_string(), "NZD".to_string()));
    data.insert("nicaragua".to_string(), ("🇳🇮".to_string(), "NIO".to_string()));
    data.insert("niger".to_string(), ("🇳🇪".to_string(), "XOF".to_string()));
    data.insert("nigeria".to_string(), ("🇳🇬".to_string(), "NGN".to_string()));
    data.insert("north macedonia".to_string(), ("🇲🇰".to_string(), "MKD".to_string()));
    data.insert("macedonia".to_string(), ("🇲🇰".to_string(), "MKD".to_string()));
    data.insert("norway".to_string(), ("🇳🇴".to_string(), "NOK".to_string()));
    
    // O
    data.insert("oman".to_string(), ("🇴🇲".to_string(), "OMR".to_string()));
    
    // P
    data.insert("pakistan".to_string(), ("🇵🇰".to_string(), "PKR".to_string()));
    data.insert("palau".to_string(), ("🇵🇼".to_string(), "USD".to_string()));
    data.insert("palestine".to_string(), ("🇵🇸".to_string(), "ILS".to_string()));
    data.insert("panama".to_string(), ("🇵🇦".to_string(), "PAB".to_string()));
    data.insert("papua new guinea".to_string(), ("🇵🇬".to_string(), "PGK".to_string()));
    data.insert("paraguay".to_string(), ("🇵🇾".to_string(), "PYG".to_string()));
    data.insert("peru".to_string(), ("🇵🇪".to_string(), "PEN".to_string()));
    data.insert("philippines".to_string(), ("🇵🇭".to_string(), "PHP".to_string()));
    data.insert("poland".to_string(), ("🇵🇱".to_string(), "PLN".to_string()));
    data.insert("portugal".to_string(), ("🇵🇹".to_string(), "EUR".to_string()));
    
    // Q
    data.insert("qatar".to_string(), ("🇶🇦".to_string(), "QAR".to_string()));
    
    // R
    data.insert("romania".to_string(), ("🇷🇴".to_string(), "RON".to_string()));
    data.insert("russia".to_string(), ("🇷🇺".to_string(), "RUB".to_string()));
    data.insert("russian federation".to_string(), ("🇷🇺".to_string(), "RUB".to_string()));
    data.insert("rwanda".to_string(), ("🇷🇼".to_string(), "RWF".to_string()));
    
    // S
    data.insert("saint kitts and nevis".to_string(), ("🇰🇳".to_string(), "XCD".to_string()));
    data.insert("saint lucia".to_string(), ("🇱🇨".to_string(), "XCD".to_string()));
    data.insert("saint vincent and the grenadines".to_string(), ("🇻🇨".to_string(), "XCD".to_string()));
    data.insert("samoa".to_string(), ("🇼🇸".to_string(), "WST".to_string()));
    data.insert("san marino".to_string(), ("🇸🇲".to_string(), "EUR".to_string()));
    data.insert("sao tome and principe".to_string(), ("🇸🇹".to_string(), "STN".to_string()));
    data.insert("saudi arabia".to_string(), ("🇸🇦".to_string(), "SAR".to_string()));
    data.insert("senegal".to_string(), ("🇸🇳".to_string(), "XOF".to_string()));
    data.insert("serbia".to_string(), ("🇷🇸".to_string(), "RSD".to_string()));
    data.insert("seychelles".to_string(), ("🇸🇨".to_string(), "SCR".to_string()));
    data.insert("sierra leone".to_string(), ("🇸🇱".to_string(), "SLL".to_string()));
    data.insert("singapore".to_string(), ("🇸🇬".to_string(), "SGD".to_string()));
    data.insert("slovakia".to_string(), ("🇸🇰".to_string(), "EUR".to_string()));
    data.insert("slovenia".to_string(), ("🇸🇮".to_string(), "EUR".to_string()));
    data.insert("solomon islands".to_string(), ("🇸🇧".to_string(), "SBD".to_string()));
    data.insert("somalia".to_string(), ("🇸🇴".to_string(), "SOS".to_string()));
    data.insert("south africa".to_string(), ("🇿🇦".to_string(), "ZAR".to_string()));
    data.insert("south sudan".to_string(), ("🇸🇸".to_string(), "SSP".to_string()));
    data.insert("spain".to_string(), ("🇪🇸".to_string(), "EUR".to_string()));
    data.insert("sri lanka".to_string(), ("🇱🇰".to_string(), "LKR".to_string()));
    data.insert("sudan".to_string(), ("🇸🇩".to_string(), "SDG".to_string()));
    data.insert("suriname".to_string(), ("🇸🇷".to_string(), "SRD".to_string()));
    data.insert("sweden".to_string(), ("🇸🇪".to_string(), "SEK".to_string()));
    data.insert("switzerland".to_string(), ("🇨🇭".to_string(), "CHF".to_string()));
    data.insert("syria".to_string(), ("🇸🇾".to_string(), "SYP".to_string()));
    
    // T
    data.insert("tajikistan".to_string(), ("🇹🇯".to_string(), "TJS".to_string()));
    data.insert("tanzania".to_string(), ("🇹🇿".to_string(), "TZS".to_string()));
    data.insert("thailand".to_string(), ("🇹🇭".to_string(), "THB".to_string()));
    data.insert("timor-leste".to_string(), ("🇹🇱".to_string(), "USD".to_string()));
    data.insert("east timor".to_string(), ("🇹🇱".to_string(), "USD".to_string()));
    data.insert("togo".to_string(), ("🇹🇬".to_string(), "XOF".to_string()));
    data.insert("tonga".to_string(), ("🇹🇴".to_string(), "TOP".to_string()));
    data.insert("trinidad and tobago".to_string(), ("🇹🇹".to_string(), "TTD".to_string()));
    data.insert("tunisia".to_string(), ("🇹🇳".to_string(), "TND".to_string()));
    data.insert("turkey".to_string(), ("🇹🇷".to_string(), "TRY".to_string()));
    data.insert("turkmenistan".to_string(), ("🇹🇲".to_string(), "TMT".to_string()));
    data.insert("tuvalu".to_string(), ("🇹🇻".to_string(), "AUD".to_string()));
    
    // U
    data.insert("uganda".to_string(), ("🇺🇬".to_string(), "UGX".to_string()));
    data.insert("ukraine".to_string(), ("🇺🇦".to_string(), "UAH".to_string()));
    data.insert("united arab emirates".to_string(), ("🇦🇪".to_string(), "AED".to_string()));
    data.insert("uae".to_string(), ("🇦🇪".to_string(), "AED".to_string()));
    data.insert("united kingdom".to_string(), ("🇬🇧".to_string(), "GBP".to_string()));
    data.insert("uk".to_string(), ("🇬🇧".to_string(), "GBP".to_string()));
    data.insert("united states".to_string(), ("🇺🇸".to_string(), "USD".to_string()));
    data.insert("usa".to_string(), ("🇺🇸".to_string(), "USD".to_string()));
    data.insert("uruguay".to_string(), ("🇺🇾".to_string(), "UYU".to_string()));
    data.insert("uzbekistan".to_string(), ("🇺🇿".to_string(), "UZS".to_string()));
    
    // V
    data.insert("vanuatu".to_string(), ("🇻🇺".to_string(), "VUV".to_string()));
    data.insert("vatican city".to_string(), ("🇻🇦".to_string(), "EUR".to_string()));
    data.insert("vatican".to_string(), ("🇻🇦".to_string(), "EUR".to_string()));
    data.insert("venezuela".to_string(), ("🇻🇪".to_string(), "VES".to_string()));
    data.insert("vietnam".to_string(), ("🇻🇳".to_string(), "VND".to_string()));
    
    // Y
    data.insert("yemen".to_string(), ("🇾🇪".to_string(), "YER".to_string()));
    
    // Z
    data.insert("zambia".to_string(), ("🇿🇲".to_string(), "ZMW".to_string()));
    data.insert("zimbabwe".to_string(), ("🇿🇼".to_string(), "ZWL".to_string()));
    
    data
});

async fn get_country(Query(params): Query<CountryQuery>) -> Json<CountryResponse> {
    let mut results = Vec::new();
    
    // Split the based parameter by comma and process each country
    let countries: Vec<&str> = params.based.split(',').map(|s| s.trim()).collect();
    
    for country_name in countries {
        let country_lower = country_name.to_lowercase();
        
        if let Some((flag, currency_code)) = COUNTRY_DATA.get(&country_lower) {
            results.push(CountryInfo {
                country: country_name.to_string(),
                flag: flag.clone(),
                currency_code: currency_code.clone(),
            });
        }
    }
    
    Json(CountryResponse { results })
}

// Separate function to create the app router for testing
fn create_app() -> Router {
    Router::new().route("/getCountry", get(get_country))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with a route
    let app = create_app();

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to 0.0.0.0:3000");
    
    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_country_single() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 1);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[0].flag, "🇯🇵");
        assert_eq!(country_response.results[0].currency_code, "JPY");
    }

    #[tokio::test]
    async fn test_get_country_multiple() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan,korea")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 2);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[0].flag, "🇯🇵");
        assert_eq!(country_response.results[0].currency_code, "JPY");
        assert_eq!(country_response.results[1].country, "korea");
        assert_eq!(country_response.results[1].flag, "🇰🇷");
        assert_eq!(country_response.results[1].currency_code, "KRW");
    }

    #[tokio::test]
    async fn test_get_country_case_insensitive() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=JAPAN")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 1);
        assert_eq!(country_response.results[0].country, "JAPAN");
        assert_eq!(country_response.results[0].flag, "🇯🇵");
        assert_eq!(country_response.results[0].currency_code, "JPY");
    }

    #[tokio::test]
    async fn test_get_country_unknown() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=unknown")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 0);
    }

    #[tokio::test]
    async fn test_get_country_with_spaces() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan,%20korea,%20usa")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 3);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[1].country, "korea");
        assert_eq!(country_response.results[2].country, "usa");
    }

    #[tokio::test]
    async fn test_get_country_mixed_valid_invalid() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=japan,unknown,korea")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        // Should only return valid countries
        assert_eq!(country_response.results.len(), 2);
        assert_eq!(country_response.results[0].country, "japan");
        assert_eq!(country_response.results[1].country, "korea");
    }

    #[tokio::test]
    async fn test_get_country_all_supported() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=usa,uk,germany")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 3);
        assert_eq!(country_response.results[0].flag, "🇺🇸");
        assert_eq!(country_response.results[0].currency_code, "USD");
        assert_eq!(country_response.results[1].flag, "🇬🇧");
        assert_eq!(country_response.results[1].currency_code, "GBP");
        assert_eq!(country_response.results[2].flag, "🇩🇪");
        assert_eq!(country_response.results[2].currency_code, "EUR");
    }

    #[tokio::test]
    async fn test_get_country_comprehensive_coverage() {
        let app = create_app();

        // Test countries from various continents
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=afghanistan,argentina,egypt,fiji,iceland,nigeria,thailand")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 7);
        
        // Afghanistan (Asia)
        assert_eq!(country_response.results[0].country, "afghanistan");
        assert_eq!(country_response.results[0].flag, "🇦🇫");
        assert_eq!(country_response.results[0].currency_code, "AFN");
        
        // Argentina (South America)
        assert_eq!(country_response.results[1].country, "argentina");
        assert_eq!(country_response.results[1].flag, "🇦🇷");
        assert_eq!(country_response.results[1].currency_code, "ARS");
        
        // Egypt (Africa)
        assert_eq!(country_response.results[2].country, "egypt");
        assert_eq!(country_response.results[2].flag, "🇪🇬");
        assert_eq!(country_response.results[2].currency_code, "EGP");
        
        // Fiji (Oceania)
        assert_eq!(country_response.results[3].country, "fiji");
        assert_eq!(country_response.results[3].flag, "🇫🇯");
        assert_eq!(country_response.results[3].currency_code, "FJD");
        
        // Iceland (Europe)
        assert_eq!(country_response.results[4].country, "iceland");
        assert_eq!(country_response.results[4].flag, "🇮🇸");
        assert_eq!(country_response.results[4].currency_code, "ISK");
        
        // Nigeria (Africa)
        assert_eq!(country_response.results[5].country, "nigeria");
        assert_eq!(country_response.results[5].flag, "🇳🇬");
        assert_eq!(country_response.results[5].currency_code, "NGN");
        
        // Thailand (Asia)
        assert_eq!(country_response.results[6].country, "thailand");
        assert_eq!(country_response.results[6].flag, "🇹🇭");
        assert_eq!(country_response.results[6].currency_code, "THB");
    }

    #[tokio::test]
    async fn test_get_country_african_countries() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=kenya,morocco,ghana,ethiopia")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 4);
        assert_eq!(country_response.results[0].flag, "🇰🇪");
        assert_eq!(country_response.results[0].currency_code, "KES");
        assert_eq!(country_response.results[1].flag, "🇲🇦");
        assert_eq!(country_response.results[1].currency_code, "MAD");
    }

    #[tokio::test]
    async fn test_get_country_aliases() {
        let app = create_app();

        // Test common aliases
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/getCountry?based=uae,czechia,burma,vatican")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let country_response: CountryResponse = serde_json::from_str(&body_str).unwrap();

        assert_eq!(country_response.results.len(), 4);
        assert_eq!(country_response.results[0].flag, "🇦🇪"); // UAE
        assert_eq!(country_response.results[0].currency_code, "AED");
        assert_eq!(country_response.results[1].flag, "🇨🇿"); // Czechia
        assert_eq!(country_response.results[1].currency_code, "CZK");
        assert_eq!(country_response.results[2].flag, "🇲🇲"); // Burma/Myanmar
        assert_eq!(country_response.results[2].currency_code, "MMK");
        assert_eq!(country_response.results[3].flag, "🇻🇦"); // Vatican
        assert_eq!(country_response.results[3].currency_code, "EUR");
    }
}
