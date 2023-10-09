use pyo3::prelude::*;
use super::errors::{Result, ParseError};
use super::authors::{Author, AuthorsFormatter};
use serde::{Serialize, Deserialize};

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibTeXModel {
    title: String,
    author: Vec<Author>,
    journal: String,
    volume: Option<usize>,
    number: Option<usize>,
    pages: Option<String>,
    year: usize,
    publisher: Option<String>,
}

fn _split_field_and_value(s: &str, lineno: usize) -> Result<(&str, &str)>{
    let s = {
        let s = s.trim();
        if s.ends_with(',') {
            &s[..s.len()-1]
        } else {
            s
        }
    };
    match s.split_once("=") {
        Some((field, value)) => {
            if value.starts_with('{') && value.ends_with('}') {
                Ok((field, &value[1..value.len()-1]))
            } else {
                Err(
                    ParseError::from_str(
                        &format!(
                            "Value is not enclosed in braces: {} (line {})",
                            s, lineno
                        )
                    )
                )
            }
        },
        None => Err(
            ParseError::from_str(
                &format!(
                    "Not in 'key=value' format: {} (line {})",
                    s, lineno
                )
            )
        ),
    }
}

impl BibTeXModel {
    pub fn from_string(s: &str) -> Result<BibTeXModel> {
        let mut title = String::new();
        let mut authors = Vec::new();
        let mut journal = String::new();
        let mut volume = None;
        let mut number = None;
        let mut pages = None;
        let mut year = 0;
        let mut publisher = None;
        // title={Structural insight into TPX2-stimulated microtubule assembly},
        for (lineno, line) in s.split("\n").enumerate() {
            if line.starts_with("@") || line.starts_with("}"){
                continue;
            }
            let (key, value) = _split_field_and_value(line.trim(), lineno)?; // check if line is in correct format (field={value}
            match key {
                "title" => title = value.to_string(),
                "author" => {
                    let author_strs: Vec<&str> = value.split(" and ").collect();
                    for s in author_strs {
                        authors.push(Author::from_string(s)?);
                    }
                },
                "journal" => journal = value.to_string(),
                "volume" => volume = Some(value.parse::<usize>()?),
                "number" => number = Some(value.parse::<usize>()?),
                "pages" => pages = Some(value.to_string()),
                "year" => year = value.parse::<usize>()?,
                "publisher" => publisher = Some(value.to_string()),
                _ => {},
            }
        }
        if title.is_empty() {
            return Err(ParseError::from_str("'title' field not found"));
        }
        if authors.is_empty() {
            return Err(ParseError::from_str("'author' field not found"));
        }
        if journal.is_empty() {
            return Err(ParseError::from_str("'journal' field not found"));
        }
        if year == 0 {
            return Err(ParseError::from_str("'year' field not found"));
        }
        Ok(BibTeXModel { title, author: authors, journal, volume, number, pages, year, publisher })
    }

    pub fn to_bibtex(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("title = {{{}}},\n", self.title));
        s.push_str(&format!("author = {{{}}},\n", self.author.iter().map(|a| a.format("f l m")).collect::<Vec<String>>().join(" and ")));
        s.push_str(&format!("journal = {{{}}},\n", self.journal));
        if let Some(volume) = self.volume {
            s.push_str(&format!("volume = {{{}}},\n", volume));
        }
        if let Some(number) = self.number {
            s.push_str(&format!("number = {{{}}},\n", number));
        }
        if let Some(pages) = &self.pages {
            s.push_str(&format!("pages = {{{}}},\n", pages));
        }
        s.push_str(&format!("year = {{{}}},\n", self.year));
        if let Some(publisher) = &self.publisher {
            s.push_str(&format!("publisher = {{{}}},\n", publisher));
        }
        s
    }

    /// Format BibTeX into a string.
    /// # Arguments
    /// * `fmt` - The format string. Something like "{authors} \"{title}\" ({year})"
    pub fn format(&self, formatter: &BibTeXFormatter) -> String {
        formatter.fmt.replace("{authors}", &formatter.authors.format(&self.author))
            .replace("{title}", &self.title)
            .replace("{journal}", &self.journal)
            .replace("{volume}", &self.volume.unwrap_or(0).to_string())
            .replace("{number}", &self.number.unwrap_or(0).to_string())
            .replace("{pages}", &self.pages.clone().unwrap_or("".to_string()))
            .replace("{year}", &self.year.to_string())
            .replace("{publisher}", &self.publisher.clone().unwrap_or("".to_string()))
    }
}


#[derive(Serialize, Deserialize)]
pub struct BibTeXFormatter {
    fmt: String,
    authors: AuthorsFormatter,
}

impl BibTeXFormatter {
    pub fn new(fmt: String, authors: AuthorsFormatter) -> BibTeXFormatter {
        BibTeXFormatter { fmt, authors }
    }

    pub fn from_string(s: &str) -> Result<BibTeXFormatter> {
        let formatter: BibTeXFormatter = match serde_json::from_str(&s) {
            Ok(formatter) => formatter,
            Err(err) => return Err(ParseError::new(format!("Error parsing JSON: {}", err))),
        };
        Ok(formatter)
    }

    pub fn from_json(path: &std::path::Path) -> Result<BibTeXFormatter> {
        let s = std::fs::read_to_string(path)?;
        Self::from_string(&s)
    }
}