use orcid::*;

// This is just for testing

fn main() {
    let client = Client::new();
    let author = client.author(&"0000-0001-5916-0947".to_string()).unwrap();
    //let author = client.author(&"0000-0003-3386-3972".to_string()).unwrap();
    println!("{}", author.credit_name().unwrap());
    println!("{}", author.full_name().unwrap());
    println!("{:?}", author.other_names());
    println!("{:?}", author.biography());
    println!("{:?}", author.researcher_urls());
    println!("{:?}", author.external_ids());
    println!("{:?}", author.keywords());
    println!("{:?}", author.education());
    println!("{:?}", author.employment());
    //println!("{:?}", author.works());
    /*
        println!(
            "{:?}",
            client
                .search_doi(&"10.1038/NATURE11174".to_string())
                .unwrap()
        );
    */
}
