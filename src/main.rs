use clap::{App, Arg};
use colored::*;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use selectors::attr::CaseSensitivity;
use selectors::Element; 
use std::error::Error;
use std::fmt;

// custom error type for the application
#[derive(Debug)]
enum AppError {
    InvalidExploit(String),
    MissingFlags,
    FetchFailed(String),
    ParseFailed,
    SelectorNotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidExploit(e) => write!(f, "Invalid exploit: {}", e),
            AppError::MissingFlags => write!(f, "Error: make sure you set bins and an exploit"),
            AppError::FetchFailed(url) => write!(f, "Failed to fetch URL: {}", url),
            AppError::ParseFailed => write!(f, "Error parsing HTML"),
            AppError::SelectorNotFound => write!(f, "Selector not found"),
        }
    }
}

impl Error for AppError {}

// valid exploits that can be used with the tool
const VALID_EXPLOITS: [&str; 15] = [
    "bind-shell",
    "capabilities",
    "bin",
    "file-download",
    "file-read",
    "file-upload",
    "file-write",
    "library-load",
    "limited-suid",
    "non-interactive-bind-shell",
    "non-interactive-reverse-shell",
    "reverse-shell",
    "shell",
    "sudo",
    "suid",
];

// struct to hold command line arguments
struct Flags {
    bins: String,
    exploit: String,
}

fn main() {
    // print the banner if no arguments are provided
    if std::env::args().len() <= 1 {
        print_banner();
        return;
    }

    // parse command line arguments
    match get_flags() {
        Ok(flags) => {
            // validate that required flags are provided and valid
            if let Err(e) = validate_required_flag_values(&flags.bins, &flags.exploit) {
                print_banner();
                println!("{}", e.to_string().red());
                return;
            }

            print_flags_banner(&flags.exploit, &flags.bins);

            let bins_list: Vec<&str> = flags.bins.split(',').map(|s| s.trim()).collect();
            
            // process each binary
            for bin in bins_list {
                let url = format!("https://gtfobins.github.io/gtfobins/{}", bin);
                if let Err(e) = print_bins(&url, bin, &flags.exploit) {
                    eprintln!("{}", e);
                }
            }

            print_credits();
        }
        Err(e) => {
            print_banner();
            println!("{}", e.to_string().red());
        }
    }
}

// print the main banner with usage information
fn print_banner() {
    println!(
        "\n{}\n",
        r#" ____  _  _  ____  ____  _  _  ____  __  __ _  ____ 
(  _ \/ )( \/ ___)(_  _)( \/ )(  _ \(  )(  ( \/ ___)
 )   /) \/ (\___ \  )(   )  /  ) _ ( )( /    /\___ \
(__\_)\____/(____/ (__) (__/  (____/(__)\_)__)(____/"#
        .truecolor(186, 218, 85)
    );
    println!("\nSearch GTFOBins without needing to open a browser\n");
    println!(
        "{}",
        "Usage: rustybins --exploit suid --bins bash,cat\n".truecolor(186, 218, 85)
    );
    println!("Available exploits:\n");
    for exploit in VALID_EXPLOITS {
        println!("- {}", exploit);
    }
    println!();
}

// parse command line arguments using clap
fn get_flags() -> Result<Flags, Box<dyn Error>> {
    let matches = App::new("rustybins")
        .about("Search GTFOBins without needing to open a browser")
        .arg(
            Arg::with_name("bins")
                .long("bins")
                .help("Comma-separated list of Bins to find given exploit for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("exploit")
                .long("exploit")
                .help("Exploit type (e.g., suid, sudo, shell)")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let bins = matches.value_of("bins").unwrap_or("").to_string();
    let exploit = matches.value_of("exploit").unwrap_or("").to_string();

    Ok(Flags { bins, exploit })
}

// validate that required flags are provided and valid
fn validate_required_flag_values(bins: &str, exploit: &str) -> Result<(), AppError> {
    if bins.is_empty() || exploit.is_empty() {
        return Err(AppError::MissingFlags);
    }

    if !is_valid_exploit(exploit) {
        return Err(AppError::InvalidExploit(exploit.to_string()));
    }

    Ok(())
}

// check if the provided exploit is valid
fn is_valid_exploit(exploit: &str) -> bool {
    VALID_EXPLOITS.contains(&exploit)
}

// print information about the flags being used
fn print_flags_banner(exploit: &str, bins: &str) {
    println!("\n---------------------------------");
    println!("{} {}", " EXPLOIT:".yellow(), exploit);
    println!("{} {}", " BINS:".yellow(), bins);
    println!("---------------------------------\n");
}

// fetch and display information about a specific binary and exploit
fn print_bins(url: &str, bin: &str, exploit: &str) -> Result<(), Box<dyn Error>> {
    // fetch the GTFOBins page
    let response = get(url).map_err(|_| AppError::FetchFailed(url.to_string()))?;
    
    if !response.status().is_success() {
        return Err(Box::new(AppError::FetchFailed(format!(
            "{}, status: {}",
            url,
            response.status()
        ))));
    }

    // parse the HTML content
    let body = response.text()?;
    let document = Html::parse_document(&body);
    
    // print the content if found
    print_content(&document, bin, exploit, url)?;

    Ok(())
}

// process and display the content from the scraped page
fn print_content(document: &Html, bin: &str, exploit: &str, url: &str) -> Result<(), Box<dyn Error>> {
    let id = format!("#{}", exploit);
    let section_selector = Selector::parse(&id).map_err(|_| AppError::ParseFailed)?;
    
    // check if the section exists
    if document.select(&section_selector).next().is_none() {
        println!("{} {} not found", "✘".red(), bin);
        return Ok(());
    }

    // print information about the found exploit
    print_title(exploit, bin, url);
    print_description(document, &id)?;
    print_examples(document, &id)?;

    Ok(())
}

// print the title of the exploit
fn print_title(exploit: &str, bin: &str, url: &str) {
    let exploit_id = exploit.replace(" ", "-");
    println!(
        "\n{} {} {}/#{}\n",
        "✔".truecolor(255, 0, 102),
        bin.truecolor(255, 0, 102),
        url,
        exploit_id
    );
}

// print the description of the exploit
fn print_description(document: &Html, id: &str) -> Result<(), Box<dyn Error>> {
    let section_selector = Selector::parse(id).map_err(|_| AppError::ParseFailed)?;
    
    // find the section and get its next siblings until we reach .examples
    if let Some(section) = document.select(&section_selector).next() {
        let mut current = section.next_sibling();
        
        while let Some(node) = current {
            if let Some(element) = node.value().as_element() {
                // check if the element has the class "examples"
                if element.has_class("examples", CaseSensitivity::CaseSensitive) {
                    break;
                }
                
                if element.name() == "p" {
                    if let Some(elem) = node.first_child() {
                        if let Some(text) = elem.value().as_text() {
                            println!("\n{}\n", text.trim());
                        }
                    }
                }
            }
            
            current = node.next_sibling();
        }
    }
    
    Ok(())
}

// print code examples for the exploit
fn print_examples(document: &Html, id: &str) -> Result<(), Box<dyn Error>> {
    // find the examples section - it should be after the section with the given id
    let section_selector = Selector::parse(id).map_err(|_| AppError::ParseFailed)?;
    let examples_selector = Selector::parse(".examples").map_err(|_| AppError::ParseFailed)?;
    let li_selector = Selector::parse("li").map_err(|_| AppError::ParseFailed)?;
    let p_selector = Selector::parse("p").map_err(|_| AppError::ParseFailed)?;
    let code_selector = Selector::parse("pre code").map_err(|_| AppError::ParseFailed)?;
    
    // Find the section element for this exploit
    if let Some(section) = document.select(&section_selector).next() {
        // Find the examples section by looking for elements with the "examples" class
        if let Some(examples) = document.select(&examples_selector).next() {
            // Process each list item
            for li in examples.select(&li_selector) {
                // Print description text
                for p in li.select(&p_selector) {
                    println!("{}", p.text().collect::<String>().trim());
                }
                
                // Print code blocks
                for code in li.select(&code_selector) {
                    println!(
                        "{}\n",
                        code.text()
                            .collect::<String>()
                            .trim()
                            .truecolor(186, 218, 85)
                    );
                }
            }
        }
    }
    
    Ok(())
}

// print credits at the end
fn print_credits() {
    println!("\n--------------------------------------------------------------------------------------------\n");
    println!("- Contribute to GTFOBins https://gtfobins.github.io/contribute/");
    println!("- Follow GTFOBins' creators https://twitter.com/norbemi https://twitter.com/cyrus_and");
    println!("- Follow the original tool creator https://twitter.com/nightshiftc");
    println!("- Follow the CristinaSolana https://github.com/CristinaSolana");
    println!("- Checkout the original Go implementation https://github.com/CristinaSolana/ggtfobins")
}
