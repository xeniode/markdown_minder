use std::fs::{self, File};
use std::io::{self, Read, Write};
use clap::Parser;
use std::collections::HashMap;
//use std::env;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file name
    #[arg(short, long)]
    output: String,

    /// Optional string argument
    #[arg(short, long)]
    string: Option<String>,

    /// Template file name for YAML frontmatter
    #[arg(long)]
    template: Option<PathBuf>,

    /// Frontmatter key-value pairs
    #[arg(long = "frontmatter", value_delimiter = ',', use_value_delimiter = true)]
    frontmatter: Option<Vec<String>>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read from stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Prepare content
    let mut content = String::new();

    // Initialize frontmatter HashMap
    let mut frontmatter = HashMap::new();

    // If a template is provided, parse its frontmatter
    if let Some(template_path) = &args.template {
        let template_content = fs::read_to_string(template_path)?;
        frontmatter.extend(parse_frontmatter(&template_content)?);
        content.push_str(&get_content_body(&template_content));
    }

    // Prepare frontmatter from --frontmatter flag
    if let Some(frontmatter_args) = &args.frontmatter {
        for pair in frontmatter_args {
            let parts: Vec<&str> = pair.splitn(2, '=').collect();
            if parts.len() == 2 {
                frontmatter.insert(parts[0].to_string(), parts[1].to_string());
            }
        }
    }

    // Serialize the updated frontmatter and prepend it to the content
    if !frontmatter.is_empty() {
        content = serialize_frontmatter(&frontmatter)? + &content;
    }

    // Add optional string argument as a title
    if let Some(s) = &args.string {
        content.push_str(&format!("# {}\n\n", s));
    }

    // Append the stdin buffer to the content
    content.push_str(&buffer);

    // Write to file
    let mut file = File::create(&args.output)?;
    file.write_all(content.as_bytes())?;

    println!("Content saved to {}", args.output);

    Ok(())
}

// Function to parse the frontmatter from the template content
fn parse_frontmatter(template_content: &str) -> io::Result<HashMap<String, String>> {
    let mut frontmatter_map = HashMap::new();
    if let Some(frontmatter_section) = template_content.split("---\n").nth(1) {
        for line in frontmatter_section.lines() {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                frontmatter_map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
            }
        }
    }
    Ok(frontmatter_map)
}

// Function to serialize the frontmatter to a string
fn serialize_frontmatter(frontmatter: &HashMap<String, String>) -> io::Result<String> {
    let mut frontmatter_string = String::from("---\n");
    for (key, value) in frontmatter {
        frontmatter_string.push_str(&format!("{}: {}\n", key, value));
    }
    frontmatter_string.push_str("---\n");
    Ok(frontmatter_string)
}

// Function to get the body content from the template content, excluding the frontmatter
fn get_content_body(template_content: &str) -> String {
    let sections: Vec<&str> = template_content.split("---\n").collect();
    if sections.len() > 2 {
        sections[2..].join("---\n")
    } else {
        String::new()
    }
}
