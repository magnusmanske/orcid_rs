use orcid::*;

// This is just for testing

fn main() {
    let client = Client::new();
    let author = client.author(&"0000-0001-5916-0947".to_string()).unwrap();
    println!("{}", author.credit_name().unwrap());
    println!("{:?}", author.external_ids());
    println!("{:?}", author.keywords());
    println!("{:?}", author.works());
    /*
        println!(
            "{:?}",
            client
                .search_doi(&"10.1038/NATURE11174".to_string())
                .unwrap()
        );
    */
}
