use regex::Regex;
use serde_json::{json, Value};
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use encoding_rs::WINDOWS_1252;

#[derive(Debug)]
struct Software {
    name: String,
    vendor: String,
    version: String,
    identifying_number: String,
    type: String,
}

impl Software {
    pub fn new(name: String, vendor: String, version: String, identifying_number: String, type: String) -> Self {
        Self {
            name,
            vendor,
            version,
            identifying_number,
            type,
        }
    }
}

pub fn parse_wmic_output(output: &str) -> Result<Vec<Software>, String> {
    let re = Regex::new(r#"(?P<name>.*) (?P<vendor>.*) (?P<version>.*) (?P<identifying_number>.*)"#).unwrap();
    let headers: Vec<String> = re.captures(output.lines().next().unwrap().trim()).unwrap().iter().map(|m| m.name(0).unwrap().to_string()).collect();

    let mut software_list = Vec::new();

    for line in output.lines().skip(1) {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }

        let values: Vec<String> = re.captures(trimmed_line).unwrap().iter().map(|m| m.name(0).unwrap().to_string()).collect();
        if values.len() != headers.len() {
            return Err(format!("Warning: Line does not match header length: {}", line));
        }

        let software = Software::new(
            values[headers.find("Caption").unwrap()].trim(),
            values[headers.find("Vendor").unwrap()].trim(),
            values[headers.find("Version").unwrap()].trim(),
            values[headers.find("IdentifyingNumber").unwrap()].trim(),
            "wmic",
        );
        software_list.push(software);
    }

    Ok(software_list)
}

pub fn parse_wing_output(output: &str) -> Result<Vec<Software>, String> {
    let prefix = "Installed package is not available from any source: ";
    let mut software_list = Vec::new();

    for line in output.lines() {
        if line.starts_with(prefix) {
            let name = line.trim_start_matches(prefix);
            let software = Software::new(name, "", "", "", "wing");
            software_list.push(software);
        }
    }

    Ok(software_list)
}

pub fn parse_hklm_output(output: &str) -> Result<Vec<Software>, String> {
    let re = Regex::new(r#"\s{2,}(?P<name>.*)\s(?P<version>.*)\s(?P<vendor>.*)"#).unwrap();
    let mut software_list = Vec::new();

    for line in output.lines().skip(2) {
        if line.len() >= 3 {
            let name = &line[0..67].trim();
            let version = &line[79..94].trim();
            let vendor = &line[94..103].trim();
            let software = Software::new(name, vendor, version, "", "hklm");
            software_list.push(software);
        }
    }

    Ok(software_list)
}

pub fn parse_appx_output(output: &str) -> Result<Vec<Software>, String> {
    let re = Regex::new(r#"\s{2,}(?P<name>.*)\s(?
