use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "the Boarische Webserver")]
enum Opt {
    Servus,
    Beers(Beers),
}

#[derive(StructOpt, Debug)]
enum Beers {
    List,
    Add(Beer),
}

#[derive(StructOpt, Deserialize, Serialize, Debug)]
struct Beer {
    #[structopt(short, long)]
    brewery: String,
    #[structopt(short, long)]
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Opt::from_args() {
        Opt::Servus => {
            let servus = reqwest::get("http://localhost:8080/").await?.text().await?;
            println!("{}", servus);
        }

        Opt::Beers(Beers::List) => {
            let beers = reqwest::get("http://localhost:8080/beers")
                .await?
                .json::<Vec<Beer>>()
                .await?;
            for Beer { brewery, name } in beers {
                println!("{} from {}", name, brewery);
            }
        }

        Opt::Beers(Beers::Add(beer)) => {
            let client = reqwest::Client::new();
            client
                .post("http://localhost:8080/beers")
                .json(&beer)
                .send()
                .await?;
            println!("Added {} from {}", beer.name, beer.brewery);
        }
    }

    Ok(())
}
