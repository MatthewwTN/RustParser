extern crate regex;

use std::env;
use std::fs;

use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenEdition {
    DATA,
    INPUT,
    PROCESS,
    OUTPUT,
    END,
    ID,
    NUM,
    TRUE,
    FALSE,
    READ,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    VECTOR,
    NUMBER,
    REGRESSIONA,
    REGRESSIONB,
    MEAN,
    STDDEV,
    CORRELATION,
    STRING,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenEdition,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenEdition, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug)]
struct Store {
    source_code: String,
    current_index: usize,
    tokens: Vec<Token>,
    scheme_output: Vec<String>,
    pl_output: Vec<String>,
    language_choice: String,
}

impl Store {
    pub fn new(source_code: String, choice: String) -> Self {
        Self {
            language_choice: choice,
            source_code: source_code.chars().collect(),
            current_index: 0,
            tokens: Vec::new(),
            scheme_output: Vec::new(),
            pl_output: Vec::new(),
        }
    }

    pub fn lex_and_parse_source(&mut self) {
        let mut temp = String::from("");
        let mut in_quote = false; // a bool value if inside a quote
        let mut quote_count = 0; // a tracker on how many quotes have been counted (max will be 2, min 0)
        let mut in_parens = false; // bool if inside a parenthesis

        let mut number_added = false; // If we have a number, it can be either 1 or more digits
        let mut num_length = 0; // These variables will help continue a loop if a number has a following number
        let mut check_length = 0;



        println!("Lexical Analysis Beginning...");

        let value: Vec<char> = self.source_code.chars().collect();

        for mut i in 0..value.len() {
            let c = value[i];

            if number_added {
                check_length += 1;
                if check_length == num_length {
                    number_added = false;
                    num_length = 0;
                    check_length = 0;
                } else {
                    continue;
                }
            }

            if c == 0xA as char {
                // If we reach a newline, and our concat string is not empty, we have an ID value
                if !temp.is_empty() {
                    self.tokens
                        .push(Token::new(TokenEdition::ID, temp.to_string()));
                    temp.clear();
                }
                temp.clear();
                continue;
            }
            if c == ' ' {
                // If we get to an empty char, skip the loop and continue (unless inside of a quote)
                if in_quote {
                    temp.push(c);
                } else {
                    continue;
                }
            }

            if c == 34 as char {
                // if we find a char that is an quote ", we are inside of a quote
                temp.push(c);
                quote_count += 1;
                // println!("Found an open quote ... {} ... {}" , temp, quote_count);
                in_quote = true;
                if quote_count == 2 {
                    // If we find a second quote, this means that our string has been enclosed and we push it to our tokens
                    let mut string_valid = temp.clone();

                    string_valid.remove(0);
                    string_valid.pop();

                    self.string_check(string_valid);

                        self.tokens
                        .push(Token::new(TokenEdition::STRING, temp.to_string()));
                        temp.clear();
                        in_quote = false;
                        quote_count = 0;
                    
                }
                continue;
            }

            temp.push(c);

            if c == ',' && !in_quote {
                if in_parens {
                    temp.pop();
                    // in some cases, after reaching a comma, we can either have a truthy value (t/f) or an ID
                    // or simply no value, these cases handle those possibilities.
                    if temp == "false" {
                        self.tokens
                            .push(Token::new(TokenEdition::FALSE, temp.to_string()));
                    } else if temp == "true" {
                        self.tokens
                            .push(Token::new(TokenEdition::TRUE, temp.to_string()));
                    } else if !temp.is_empty() {
                        self.tokens
                            .push(Token::new(TokenEdition::ID, temp.to_string()));
                    }
                    temp.clear();
                    self.tokens
                        .push(Token::new(TokenEdition::COMMA, c.to_string()));
                    continue;
                }

                // if not inside a parentheses, this will handle the cases
                temp.pop();
                if !temp.is_empty() {
                    let mut check_temp = temp.clone();
                    check_temp = check_temp.to_lowercase();
                    if check_temp != temp {
                        panic!(
                            "Found an invalid value!  {}  is not accepted as a value",
                            temp
                        );
                    }
                    self.tokens
                        .push(Token::new(TokenEdition::ID, temp.to_string()));
                }

                self.tokens
                    .push(Token::new(TokenEdition::COMMA, c.to_string()));
                temp.clear();
            }
            if c == '(' {
                self.tokens
                    .push(Token::new(TokenEdition::LPAREN, c.to_string()));
                in_parens = true;
                temp.clear();
            }
            if c == ')' {
                temp.pop();
                if !temp.is_empty() && temp.len() > 1 {
                    self.tokens
                        .push(Token::new(TokenEdition::ID, temp.to_string()));
                }
                self.tokens
                    .push(Token::new(TokenEdition::RPAREN, c.to_string()));
                in_parens = false;
                temp.clear();
            }
            if c == '.' && !in_quote {
                self.tokens
                    .push(Token::new(TokenEdition::PERIOD, c.to_string()));
                temp.clear();
            }
            if c == '=' && !in_quote {
                temp.pop();
                if !temp.is_empty() {
                    self.tokens
                        .push(Token::new(TokenEdition::ID, temp.to_string()));
                }

                self.tokens
                    .push(Token::new(TokenEdition::ASSIGN, c.to_string()));
                temp.clear();
            }
            // if any of these key words are found, this will create a token for them and add it to our vector of tokens
            if temp == "data" {
                self.tokens
                    .push(Token::new(TokenEdition::DATA, temp.to_string()));
                // println!("{}" , temp);
                temp.clear();
            }
            if temp == "input" {
                self.tokens
                    .push(Token::new(TokenEdition::INPUT, temp.to_string()));
                // println!("{}" , temp);
                temp.clear();
            }
            if temp == "process" {
                self.tokens
                    .push(Token::new(TokenEdition::PROCESS, temp.to_string()));
                temp.clear();
            }
            if temp == "output" {
                self.tokens
                    .push(Token::new(TokenEdition::OUTPUT, temp.to_string()));
                temp.clear();
            }
            if temp == "end" {
                self.tokens
                    .push(Token::new(TokenEdition::END, temp.to_string()));
                temp.clear();
            }
            if c == ':' {
                temp.pop();
                if !temp.is_empty() {
                    self.tokens
                        .push(Token::new(TokenEdition::ID, temp.to_string()));
                    temp.clear();
                }
                self.tokens
                    .push(Token::new(TokenEdition::COLON, c.to_string()));
            }
            // If we reach any of the below key words, this will append them...
            if temp == "read" {
                self.tokens
                    .push(Token::new(TokenEdition::READ, temp.to_string()));
                temp.clear();
            }
            if temp == "number" {
                self.tokens
                    .push(Token::new(TokenEdition::NUMBER, temp.to_string()));
                temp.clear();
            }
            if temp == "vector" {
                self.tokens
                    .push(Token::new(TokenEdition::VECTOR, temp.to_string()));
                temp.clear();
            }
            if temp == "regressiona" {
                self.tokens
                    .push(Token::new(TokenEdition::REGRESSIONA, temp.to_string()));
                temp.clear();
            }
            if temp == "regressionb" {
                self.tokens
                    .push(Token::new(TokenEdition::REGRESSIONB, temp.to_string()));
                temp.clear();
            }
            if temp == "correlation" {
                self.tokens
                    .push(Token::new(TokenEdition::CORRELATION, temp.to_string()));
                temp.clear();
            }
            if temp == "false" {
                self.tokens
                    .push(Token::new(TokenEdition::FALSE, temp.to_string()));
                temp.clear();
            }
            if temp == "true" {
                self.tokens
                    .push(Token::new(TokenEdition::TRUE, temp.to_string()));
                temp.clear();
            }
            if temp == "stddev" {
                self.tokens
                    .push(Token::new(TokenEdition::STDDEV, temp.to_string()));
                temp.clear();
            }
            if temp == "mean" {
                self.tokens
                    .push(Token::new(TokenEdition::MEAN, temp.to_string()));
                temp.clear();
            }
            if c.is_numeric() && !in_quote {
                // a numeric value symbolizes a value within the read function, this will add that value as a token
                number_added = true;
                let mut temp_builder = String::from("");
                while value[i].is_numeric() {
                    temp_builder.push(value[i]);
                    i += 1;
                    num_length += 1;
                }
                self.tokens
                    .push(Token::new(TokenEdition::NUM, temp_builder.to_string()));
                temp.clear();
            }
        }

        
        self.program_syntax();
    }

    fn get_next_token(&mut self) -> &Token {
        self.current_index += 1;
        let toke = &self.tokens[self.current_index];
        return toke;
    }

    fn cur_token(&self) -> &Token {
        return &self.tokens[self.current_index];
    }

    fn peek(&mut self) -> &Token {
        return &self.tokens[self.current_index + 1];
    }

    fn program_syntax(&mut self) {
        assert!(self.cur_token().kind == TokenEdition::DATA); // Every programs token should begin with Data
        assert!(self.get_next_token().kind == TokenEdition::COLON);  // Followed by a colon

        // Below, the following functions will begin
        self.data_defs();
        self.input_ops();
        self.process_ops();
        self.output_ops();

        println!("Syntax Analysis Completed");

        if self.language_choice == "-s" {
            for val in &self.scheme_output {
                println!("{}", val);
            }
        } else if self.language_choice == "-p" {
            let pro_log = String::from("main :-");
            println!("  {}", pro_log);
            for val in &self.pl_output {
                println!("\t{}", val);
            }
        } else {
            println!(
                "You entered in {}, please choose either '-p' for prolog or '-s' for Scheme",
                self.language_choice
            );
        }
    }

    // Data defs will call data def, and while there is a comma remaining after the call as the next token, we know there should be another data definition
    fn data_defs(&mut self) {
        self.data_def();
        while self.get_next_token().kind == TokenEdition::COMMA {
            self.data_def();
        }
        // at the end, we assume since there was no comma after a datadef, the input section should follow.
        assert!(self.cur_token().kind == TokenEdition::INPUT);
        assert!(self.get_next_token().kind == TokenEdition::COLON);
    }

    fn data_def(&mut self) {
        // We should have a token with an ID value
        assert!(self.get_next_token().kind == TokenEdition::ID);
        self.id_check(self.cur_token().kind, self.cur_token().literal.clone()); // Checking ID Lexically
        // then a colon
        assert!(self.get_next_token().kind == TokenEdition::COLON);
        // Then either a number of a vector
        assert!(
            self.get_next_token().kind == TokenEdition::NUMBER
                || self.cur_token().kind == TokenEdition::VECTOR
        ); 
    }

    fn input_ops(&mut self) {
        self.input_op();
        while self.get_next_token().kind == TokenEdition::COMMA {
            self.input_op();
        }

        assert!(self.cur_token().kind == TokenEdition::PROCESS);
        assert!(self.get_next_token().kind == TokenEdition::COLON);
    }

    fn input_op(&mut self) {
        // When building an input op, it begins with either define or load_data_column
        // This function will parse through the input op, check the syntax, and build the string that will be placed in a vector
        let mut scheme_builder = String::from("(define ");
        let mut prolog_builder = String::from("load_data_column(");

        assert!(self.get_next_token().kind == TokenEdition::ID);
        self.id_check(self.cur_token().kind, self.cur_token().literal.clone());

        scheme_builder.push_str(&self.cur_token().literal); // Scheme output adds ID
        let pl_id = String::from(self.cur_token().literal.clone()); // saving ID for prolog

        assert!(self.get_next_token().kind == TokenEdition::ASSIGN);
        assert!(self.get_next_token().kind == TokenEdition::READ);
        assert!(self.get_next_token().kind == TokenEdition::LPAREN);

        scheme_builder.push_str(" (read-csv "); // If read and LParen are in string, this adds to Scheme

        assert!(self.get_next_token().kind == TokenEdition::STRING);

        let scheme_file = self.manip_scheme_string(self.cur_token().literal.clone());
        scheme_builder.push_str(&scheme_file); // Have to manipulate the file string in each of these...

        let temp_pl = self.cur_token().literal.clone();
        let new_string = self.manip_pl_string(temp_pl.clone());
        prolog_builder.push_str(&new_string);

        assert!(self.get_next_token().kind == TokenEdition::COMMA);

        prolog_builder.push_str(&self.cur_token().literal); // Appending commas to prolog


        let bool_check = self.get_next_token().kind == TokenEdition::TRUE
            || self.cur_token().kind == TokenEdition::FALSE;
        assert!(bool_check);
        if self.cur_token().kind == TokenEdition::TRUE {
            scheme_builder.push_str(" #t "); // true value becomes #t
            prolog_builder.push_str(" ");
            prolog_builder.push_str(&self.cur_token().literal);
        } else if self.cur_token().kind == TokenEdition::FALSE {
            scheme_builder.push_str(" #f "); // false value becomes #f 
            prolog_builder.push_str(" ");
            prolog_builder.push_str(&self.cur_token().literal);
            // prolog in both cases remains the same
        }

        assert!(self.get_next_token().kind == TokenEdition::COMMA);
        prolog_builder.push_str(&self.cur_token().literal);

        assert!(self.get_next_token().kind == TokenEdition::NUM);

        prolog_builder.push_str(" "); // Adding Spaces for Scheme output if our number is found. . .
        prolog_builder.push_str(&self.cur_token().literal);
        scheme_builder.push_str(&self.cur_token().literal);

        assert!(self.get_next_token().kind == TokenEdition::RPAREN);
        prolog_builder.push_str(", V"); // Adding the capital V to our variable
        prolog_builder.push_str(&pl_id); // Adding our saved ID to the input string
        prolog_builder.push_str(&self.cur_token().literal); // then adding our current parentheses to the string

        scheme_builder.push_str("))");
        prolog_builder.push_str(",");

        self.scheme_output.push(scheme_builder);
        self.pl_output.push(prolog_builder);
        // Pushing both strings to their respective vectors...
    }


    // This function will call process ops and will continue until there is no comma remaining
    fn process_ops(&mut self) {
        self.process_op();
        while self.get_next_token().kind == TokenEdition::COMMA {
            self.process_op();
        }

        // at the end, assuming there is no comma, we know we have reached the end of the process ops and should be at the output section
        assert!(self.cur_token().kind == TokenEdition::OUTPUT);
        assert!(self.get_next_token().kind == TokenEdition::COLON);
    }


    // This function handles two flows, some functions can take 1 or 2 parameters, and based on that assertions are made for the next token
    fn process_op(&mut self) {
        let mut scheme_builder = String::from("(define ");
        let mut prolog_builder = String::from("");
        let function_kind;

        assert!(self.get_next_token().kind == TokenEdition::ID);
        self.id_check(self.cur_token().kind, self.cur_token().literal.clone()); // Checking our string with a regex

        let pl_id = self.cur_token().literal.clone();
        scheme_builder.push_str(&self.cur_token().literal);

        assert!(self.get_next_token().kind == TokenEdition::ASSIGN);
        assert!(
            self.get_next_token().kind == TokenEdition::CORRELATION
                || self.cur_token().kind == TokenEdition::MEAN
                || self.cur_token().kind == TokenEdition::REGRESSIONA
                || self.cur_token().kind == TokenEdition::REGRESSIONB
                || self.cur_token().kind == TokenEdition::STDDEV
        );

        function_kind = self.cur_token().kind;

        scheme_builder.push_str(" (");
        scheme_builder.push_str(&self.cur_token().literal);
        prolog_builder.push_str(&self.cur_token().literal); // Appending our function type to our source code...

        assert!(self.get_next_token().kind == TokenEdition::LPAREN);
        prolog_builder.push_str(&self.cur_token().literal);

        assert!(self.get_next_token().kind == TokenEdition::ID);

        // pushing id in scheme
        scheme_builder.push_str(" ");
        scheme_builder.push_str(&self.cur_token().literal);
        // Pro log pushes ID
        prolog_builder.push_str("V");
        prolog_builder.push_str(&self.cur_token().literal);
        prolog_builder.push_str(", ");

        //  println!("{:?}", self.cur_token().kind);
        if function_kind == TokenEdition::CORRELATION
            || function_kind == TokenEdition::REGRESSIONA
            || function_kind == TokenEdition::REGRESSIONB
        {
            assert!(self.get_next_token().kind == TokenEdition::COMMA);
            assert!(self.get_next_token().kind == TokenEdition::ID);

            scheme_builder.push_str(" ");
            scheme_builder.push_str(&self.cur_token().literal);

            prolog_builder.push_str("V");
            prolog_builder.push_str(&self.cur_token().literal);
            prolog_builder.push_str(", ");
        }

        if function_kind == TokenEdition::MEAN || function_kind == TokenEdition::STDDEV {
            assert!(
                self.get_next_token().kind == TokenEdition::RPAREN,
                "{:?} can only take in one parameter",
                function_kind
            );
        }

        prolog_builder.push_str("V");
        prolog_builder.push_str(&pl_id);

        if function_kind == TokenEdition::CORRELATION
            || function_kind == TokenEdition::REGRESSIONA
            || function_kind == TokenEdition::REGRESSIONB
        {
            assert!(self.get_next_token().kind == TokenEdition::RPAREN);
        }

        scheme_builder.push_str("))");
        prolog_builder.push_str("),");

        self.scheme_output.push(scheme_builder);
        self.pl_output.push(prolog_builder);
    }

    fn output_ops(&mut self) {
        // We call the output opp function, if a comma is the next token, we know another output op follows, we continue this logic until no comma remains
        self.output_op();
        while self.get_next_token().kind == TokenEdition::COMMA {
            self.output_op();
            self.scheme_output.push("(newline)".to_string());
        }

        // After there is no comma, we expect there to be an end token, signifying the end of the file.
        assert!(self.cur_token().kind == TokenEdition::END);
        assert!(self.get_next_token().kind == TokenEdition::PERIOD);
    }


    // This function will build one line of output, for an output operation in scheme and prolog
    fn output_op(&mut self) {
        let mut scheme_builder = String::from("(display ");
        let mut prolog_builder = String::from("writeln(");

        assert!(
            self.get_next_token().kind == TokenEdition::STRING
                || self.cur_token().kind == TokenEdition::ID
        );

        scheme_builder.push_str(&self.cur_token().literal);
        scheme_builder.push_str(")");

        if self.cur_token().kind == TokenEdition::ID {
            prolog_builder.push_str("V");
            prolog_builder.push_str(&self.cur_token().literal);
        } else if self.cur_token().kind == TokenEdition::STRING {
            prolog_builder.push_str(&self.cur_token().literal);
        }

        if self.peek().kind == TokenEdition::END {
            prolog_builder.push_str(").");
        } else {
            prolog_builder.push_str("),");
        }

        self.scheme_output.push(scheme_builder);
        self.pl_output.push(prolog_builder);
    }

    fn id_check(&mut self, kind: TokenEdition, value: String) {
        // an ID checker that makes sure an ID is valid
        let pattern = r"[a-z]+";
        let re = Regex::new(pattern).unwrap();
        assert!(re.is_match(&value));
        assert!(kind == TokenEdition::ID);
    }

    fn string_check (&mut self, value : String) {
       // This is a regex used to determine whether a given string is following name conventions
        let pattern = r"[a-z.0-9 = -]+";
        let re = Regex::new(pattern).unwrap();

        let binding = value.to_string();
        let check = re.find(&binding).unwrap();
        assert_eq!(check.as_str(), value);       
    }




    fn manip_pl_string(&mut self, mut val: String) -> String {
        val.pop();
        val.push('\'');
        val.remove(0);
        let prefix = '\'';

        // This function will remove the quote marks from a string and replace them with char marks example -> "Bleh ble ble " -> ' Bleh ble ble '
        let new_string = prefix.to_string() + &val;
        new_string
    }

    fn manip_scheme_string(&mut self, mut val: String) -> String {
        // Following the example of the Assignment Description, that adds as ./ before a file, this was a small thing and maybe it wasn't needed
        val.remove(0);
        let prefix = "\"./";
        let new_string = prefix.to_string() + &val;
        new_string
    }

}

fn main() {
    
    let user_file = env::args().nth(1); // Checking for the file 
    let language_choice = env::args().nth(2); // Checking for scheme or prolog

    if user_file == None {
        println!("Please re-execute program with a valid source file.") // if no file then message
    }
    else if language_choice == None {
        println!("Please enter in a valid language flag -s for scheme or -p for prolog"); // if no language then message
    }
    else {


        let file = user_file.unwrap(); // unwrapping 
        let lang = language_choice.unwrap(); // unwrapping 

        if lang != "-s" && lang != "-p" {
            println!("Please enter a valid selection or prolog -p or scheme -s");
        }
        else {
            let fetch_da = fs::read_to_string(file).unwrap();
            let mut store = Store::new(fetch_da, lang);
            store.lex_and_parse_source(); // Lexing and parsing
        }

    
    }    
}
