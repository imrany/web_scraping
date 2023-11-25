use reqwest;

fn main() {
    let response=reqwest::blocking::get("https://www.accreditationnow.com/");
    let data=response.unwrap().text().unwrap();
    println!("{}", data);
}
