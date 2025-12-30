use reqwest::blocking::get;
use serde::Deserializer;
use serde::Deserialize;
use prettytable::{Table, row};
use std::io::{self, Write};

#[derive(Deserialize)]
struct IpInfo {
    ip: Option<String>,
    city: Option<String>,
    #[serde(deserialize_with = "parse_loc")]
    //example line from resp json: 'loc	"40.7143,-74.0060"'
    loc: Option<(f64, f64)>,
    region: Option<String>,
    country: Option<String>,
    postal: Option<String>,
    timezone: Option<String>,
    org: Option<String>,
}

fn parse_loc<'de, D>(deserializer: D) -> Result<Option<(f64, f64)>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    s.map(|s| {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            Ok((parts[0].parse().unwrap(), parts[1].parse().unwrap()))
        } else {
            Err(serde::de::Error::custom("invalid location format"))
        }
    }).transpose()
}

const NA: &'static str = r" 
⠀⢨⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠓⠒⠠⣀⠀⠀⠀⠀⠀⡜⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣷⠀⠀⠀⠀
⠀⣀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⢹⠀⢪⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡼⠀⡞⠓⠋⡶⡄
⠀⠑⡝⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⡇⠀⠀⠀⠀⠀⠀⠀⡰⣊⣻⠒⠀⠘⡦⠔⠲⠛⠃
⠀⠀⢸⢾⡗⠢⠤⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀⣼⠊⠀⢣⣄⣀⡰⡄⠀⠀⠀⠀
⠀⠀⢸⠈⠁⠀⠀⠀⠀⠈⠉⠁⠒⠒⠂⠤⠤⠤⠤⠤⠤⠄⠤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠰⡟⠙⣄⣀⡭⣩⠞⠁⠀⠀⠀⠀
⠀⢠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠑⠒⣄⡀⠀⠀⠀⠀⠀⣀⡠⠴⠃⢀⠔⠙⠴⠁⠀⠀⠀⠀⠀⠀
⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⡆⠀⢀⣀⠰⠁⠀⠀⠀⣇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡌⡠⠜⠀⠀⠀⠀⣠⡴⠈⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠸⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠀⠀⠀⠀⢀⢄⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣾⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠸⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠈⠳⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡰⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⡏⢉⣢⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⠞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢱⣸⠈⡇⠉⠒⠒⠒⠒⢦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢀⡵⢳⠱⡀⠀⠀⠀⠀⠀⠱⣀⠤⢄⠀⠀⠀⠀⠀⣀⠤⠤⢄⣀⡎⠉⠉⠑⠊⠑⢄⠘⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠑⢆⢣⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠣⡀⠀⡔⠊⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⣄⠈⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠸⣌⡆⠘⢄⠀⠀⠀⠀⠀⠀⠀⠀⠓⠤⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠓⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠈⢎⠆⠈⠳⡀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠱⡀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢺⠀⠀⠀⠀⠀⠀⠀⠈⢆⠀⠀⠀⠀⠀⢰⠉⠀⠀⡎⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠦⣀⡀⠀⠀⠀⠀⠈⠢⢤⡠⠤⠔⢊⣀⣀⠴⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠒⠤⢄⡀⠀⠀⣀⠀⠀⠀⣪⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠉⠀⠉⠲⠮⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
";

// const TL: (f64, f64) = (53.74225227651084, -131.57469643966365);
// const TR: (f64, f64) = (51.887584987370666, -51.61315796477492);
// const BL: (f64, f64) = (11.146346299784845, -116.1330539698415);
// const BR: (f64, f64) = (13.29276906180568, -69.06739520789351);

// fn main() {
//     println!("{}", NA);
// }


fn run() -> Result<(), Box<dyn std::error::Error>> {
    let resp = get("https://ipinfo.io/json")?.json::<IpInfo>()?;

    let mut table = Table::new();
    table.add_row(row!["Field", "Value"]);
    table.add_row(row!["IP", resp.ip.unwrap_or("NOT FOUND".to_string())]);
    table.add_row(row!["Country", resp.country.unwrap_or("NOT FOUND".to_string())]);
    table.add_row(row!["Region", resp.region.unwrap_or("NOT FOUND".to_string())]);
    table.add_row(row!["City", resp.city.unwrap_or("NOT FOUND".to_string())]);
    table.add_row(row!["ZIP", resp.postal.unwrap_or("NOT FOUND".to_string())]);
    table.add_row(row!["Timezone", resp.timezone.unwrap_or("NOT FOUND".to_string())]);
    table.add_row(row!["Org", resp.org.unwrap_or("NOT FOUND".to_string())]);
    table.add_row(row!["loc", resp.loc.map(|c| format!("{:?}", c)).unwrap_or("NOT FOUND".to_string())]);

    table.printstd();
    Ok(())
}


fn main() {
    if let Err(e) = run() {
        eprintln!("\nError: {}", e);
    }

    println!("\npress Enter to exit...");
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}

