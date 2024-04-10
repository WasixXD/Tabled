#[derive(Debug, Eq, PartialEq)]
pub struct Table {
    cols: usize,
    rows: usize,
    padding: i8,
    pub style: Tokens,
    data: Vec<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tokens {
    pub border: String,
    pub horizontal: String,
    pub vertical: String,
}

impl Tokens {
    pub fn new(style: &str) -> Option<Self> {
        match style {
            "lines" => {
                return Some(Self {
                    border: String::from("+"),
                    horizontal: String::from("-"),
                    vertical: String::from("|"),
                })
            }

            _ => return None,
        }
    }
}

impl Table {
    pub fn new(data: Vec<Vec<String>>, style_opt: &str) -> Option<Self> {
        let cols = data.len();
        let rows = data[0].len();

    
        
        let padding = 2;
        // TODO: Umbiguos?
        let style = match style_opt {
            "lines" => Tokens::new("lines").unwrap(),
            _ => todo!(),
        };

        Some(Self {
            cols,
            rows,
            padding,
            style,
            data,
        })
    }

    fn find_largest_len(&self) -> usize {
        let mut largest: usize = self.data[0][0].len();
        for i in 0..self.cols {
            for j in 0..self.rows {
                if self.data[i][j].len() >= largest {
                    largest = self.data[i][j].len();
                }
            }
        }
        largest
    }

    fn gen_separator_lines(&self, p: usize) -> String {
        let mut line = String::new();

        for _i in 0..self.rows {
            line.push_str(&format!("{}{}", self.style.border, "-".repeat(p)));
        }

        line
    }

    fn gen_content_line(&self, row: usize, p: usize) -> String {
        let mut line = String::new();
        let current_padding = p / self.padding as usize;
        let total_padding = p * self.padding as usize;
        let white_space = " ";


        for i in 0..self.rows {
        let last_padding = ((total_padding - (current_padding + self.data[row][i].len())) as f64).ceil() as usize;
            line.push_str(&format!(
                "{}{}{}{}",
                self.style.vertical,
                white_space.repeat(current_padding),
                self.data[row][i],
                white_space.repeat(last_padding)
            ));
        }

        line
    }

    pub fn plot(&self) {
        // let white_space = " ".repeat(self.padding as usize);
        let mut c_row = 0;
        let largest_content = self.find_largest_len();
        for i in 0..self.cols {
            // print first line of table

            let line = self.gen_separator_lines(largest_content * self.padding as usize);
            println!("{line}{}", self.style.border);

            let c_line = self.gen_content_line(c_row, largest_content as usize);
            println!("{}{}", c_line, self.style.vertical);

            c_row += 1;

            // println!("{line}{}", self.style.border);

            if i == self.cols - 1 {
                println!("{line}{}", self.style.border);
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_table() {
        let mock_data = vec![
            vec!["Nome".to_string(), "Idade".to_string()],
            vec!["Lucas".to_string(), "18".to_string()],
        ];
        let table = Table::new(mock_data.clone(), "lines");

        let expected = Table {
            cols: 2,
            rows: 2,
            padding: 2,
            style: Tokens::new("lines").unwrap(),
            data: mock_data.clone(),
        };
        assert_eq!(table.unwrap(), expected);
    }

    #[test]
    fn diff_styles() {
        let style = Tokens::new("lines").unwrap();

        let tokens = Tokens {
            border: "+".to_string(),
            horizontal: "-".to_string(),
            vertical: "|".to_string(),
        };

        assert_eq!(style, tokens);
    }

    #[test]
    fn get_chars() {
        let mock_data = vec![
            vec!["Nome".to_string(), "Idade".to_string()],
            vec!["Lucas".to_string(), "18".to_string()],
        ];
        let table = Table::new(mock_data.clone(), "lines").unwrap();

        let m_style = Tokens::new("lines").unwrap();

        assert_eq!(table.style, m_style);
    }

    #[test]
    fn find_largest() {
        let mock_data = vec![
            vec!["Nome".to_string(), "Idade".to_string()],
            vec!["Lucas".to_string(), "18".to_string()],
            vec!["Dracula".to_string(), "1000".to_string()],
        ];

        let table = Table::new(mock_data.clone(), "lines").unwrap();

        let largest_len = "Dracula".len();
        let n = table.find_largest_len();

        assert_eq!(n, largest_len);
    }
}
