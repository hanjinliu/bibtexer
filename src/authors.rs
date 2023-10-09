use super::errors::{ParseError, Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    first: String,
    last: String,
    middle: String,
}

impl Author {
    pub fn from_string(s: &str) -> Result<Author> {
        let split: Vec<&str> = s.split(", ").collect();
        if split.len() != 2 {
            return Err(ParseError::new(format!("Author string does not contain comma: {}", s)));
        }
        let last = split[0].trim().to_string();
        let second: Vec<&str> = split[1].trim().split(" ").collect();
        let (first, middle) = if second.len() == 1 {
            (second[0], "")
        } else if second.len() == 2 {
            (second[0], second[1])
        } else {
            return Err(ParseError::new(format!("Author string not in correct format: {}", s)));
        };
        Ok(Author { first: first.to_string(), last, middle: middle.to_string()})
    }

    pub fn format(&self, fmt: &str) -> String {
        let mut vec = Vec::new();
        for char in fmt.chars() {
            let c = match char {
                'f' => self.first.clone(),
                'l' => self.last.clone(),
                'm' => self.middle.clone(),
                'F' => self.first.chars().next().unwrap_or('#').to_uppercase().to_string(),
                'L' => self.last.chars().next().unwrap_or('#').to_uppercase().to_string(),
                'M' => self.middle.chars().next().unwrap_or('#').to_uppercase().to_string(),
                _ => char.to_string(),
            };
            vec.push(c);
        }
        vec.join("")
    }

    pub fn has_middle(&self) -> bool {
        !self.middle.is_empty()
    }

}


#[derive(Clone, Serialize, Deserialize)]
pub enum ElideMode {
    #[serde(rename = "last")]
    Last,
    #[serde(rename = "before-last")]
    BeforeLast,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ElideSetting {
    letters: String,
    limit: usize,
    how: ElideMode,
}

impl ElideSetting {
    pub fn default() -> ElideSetting {
        ElideSetting {
            letters: " et al., ".to_string(),
            limit: 1,
            how: ElideMode::Last,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthorsFormatter {
    fmt: String,
    sep: String,
    and: String,
    elide: ElideSetting,
}

impl AuthorsFormatter {
    fn _split_fmt(&self) -> (String, String) {
        let s = &self.fmt;
        let left_idx = s.find('[').unwrap_or(s.len());
        let right_idx = s.find(']').unwrap_or(0);
        if left_idx < right_idx {
            let without_middle = if right_idx < s.len() - 1 {
                s[..left_idx].to_owned() + &s[right_idx + 1..]
            } else {
                s[..left_idx].to_owned()
            };
            let mut with_middle = s.to_string();
            with_middle.remove(right_idx);
            with_middle.remove(left_idx);
            (without_middle, with_middle)
        } else {
            (s.to_string(), s.to_string())
        }
    }

    pub fn new(s: &str) -> AuthorsFormatter {
        AuthorsFormatter {
            fmt: s.to_string(),
            sep: ", ".to_string(),
            and: " and ".to_string(),
            elide: ElideSetting::default(),
        }
    }

    pub fn with_sep(&self, sep: &str) -> AuthorsFormatter {
        Self {
            fmt: self.fmt.clone(),
            sep: sep.to_string(),
            and: self.and.to_string(),
            elide: self.elide.clone(),
        }
    }

    pub fn with_elide(&self, letters: &str, limit: usize, how: ElideMode) -> AuthorsFormatter {
        Self {
            fmt: self.fmt.clone(),
            sep: self.sep.clone(),
            and: self.and.to_string(),
            elide: ElideSetting {
                letters: letters.to_string(),
                limit,
                how,
            }
        }
    }

    pub fn format(&self, authors: &Vec<Author>) -> String {
        let mut vec = Vec::new(); // Author strings
        let (fmt, fmt_with_middle) = self._split_fmt();
        for author in authors {
            vec.push(
                if author.has_middle() {
                    author.format(&fmt_with_middle)
                } else {
                    author.format(&fmt)
                }
            );
        }
        let joined = if vec.len() > self.elide.limit {
            match self.elide.how {
                ElideMode::Last => {
                    let mut s = vec[0..self.elide.limit].join(&self.sep);
                    s.push_str(&self.elide.letters);
                    s
                },
                ElideMode::BeforeLast => {
                    let mut s = vec[0..self.elide.limit - 2].join(&self.sep);
                    s.push_str(&self.elide.letters);
                    s.push_str(&vec[vec.len() - 1]);
                    s
                }
            }
        } else if vec.len() == 2 {
            vec.join(&self.and)
        } else if vec.len() == 1{
            vec[0].clone()
        } else {
            let s0 = vec[..vec.len() - 1].join(&self.sep);
            s0 + &self.and + &vec[vec.len() - 1]
        };
        joined
    }
}
