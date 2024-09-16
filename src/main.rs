use std::fs::{self, File};
use std::io::{self, Read, Write};
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{Utc, SecondsFormat};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file name
    #[arg(short, long)]
    output: String,

    /// Title for the frontmatter
    #[arg(long)]
    title: Option<String>,

    /// Template file name for YAML frontmatter
    #[arg(long)]
    template: Option<PathBuf>,

    /// Frontmatter key-value pairs
    #[arg(long = "frontmatter", value_delimiter = ',', use_value_delimiter = true)]
    frontmatter: Option<Vec<String>>,

    /// Tags for the frontmatter
    #[arg(long = "tag")]
    tags: Vec<String>,

    /// ID for the frontmatter, set to the current Unix timestamp if --id-unix is used
    #[arg(long)]
    id: Option<String>,

    /// Flag to set the ID to the current Unix timestamp
    #[arg(long = "id-unix")]
    id_unix: bool,

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

    // Add title from --title flag to the frontmatter
    if let Some(title) = &args.title {
        frontmatter.insert("title".to_string(), title.clone());
    }

    // Set ID to the current Unix timestamp if --id-unix is used, otherwise use the provided ID
    if args.id_unix {
        let unix_timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true);
        frontmatter.insert("id".to_string(), unix_timestamp);
    } else if let Some(id) = &args.id {
        frontmatter.insert("id".to_string(), id.clone());
    }

    // Add tags from --tag flags to the frontmatter
    if !args.tags.is_empty() {
        frontmatter.insert("tags".to_string(), args.tags.join(", "));
    }

    // Serialize the updated frontmatter and prepend it to the content
    if !frontmatter.is_empty() {
        content = serialize_frontmatter(&frontmatter)? + &content;
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
        if key == "tags" {
            // Serialize tags as a YAML list
            let tags: Vec<String> = value.split(", ").map(|s| format!("- {}", s)).collect();
            frontmatter_string.push_str(&format!("{}:\n{}\n", key, tags.join("\n")));
        } else {
            frontmatter_string.push_str(&format!("{}: {}\n", key, value));
        }
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

