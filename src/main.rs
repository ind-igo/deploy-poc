use std::fs;
use std::error::Error;

use serde::{Serialize, Deserialize};
use serde_yaml;

#[derive(Deserialize, Debug)]
struct ContractDeployment {
    name: String,
    factory: String,
    address: Option<String>,
    //params: Vec
}

fn main() -> Result<(), Box<dyn Error>> {
    let data = r#"
      - name: dai
        factory: erc20
        address: "test"
    "#;

    let file = fs::File::open("deployment.yaml")?;
    let deployment: Vec<ContractDeployment> = serde_yaml::from_reader(file)?;
    //let file: Vec<ContractDeployment> = serde_yaml::from_str(data)?;

    println!("{:#?}", deployment);

    Ok(())
}
