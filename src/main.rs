use serde_derive::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Deserialize, Serialize, Debug)]
struct Dtt {
    id: u32,
    value: String,
    info: String
}

#[derive(Deserialize, Serialize)]
struct Dt {
    dtt: Vec<Dtt>
}

fn main() -> Result<(), std::io::Error> {
    let path = "../dt.json";
    
    let mut dt_data = {
        let dt_data = std::fs::read_to_string(&path)?;

        // Load the Dt structure from the string.
        serde_json::from_str::<Dt>(&dt_data).unwrap()
    };

    // Double the quantity for each element in 'dtt'
    for index in 0..dt_data.dtt.len() {
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

        println!("{:?}", dt_data.dtt);

        dt_data.dtt[index].info = rand_string;
    }

    // Save the JSON structure into the output file
    std::fs::write(
        path,
        serde_json::to_string_pretty(&dt_data).unwrap(),
    )?;

    Ok(())
}