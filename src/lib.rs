

use std::fmt::{self, Debug, Display, Formatter};

use phf::phf_map;

// #[derive(Debug)]
enum ParameterError {
    NotFound,
    FailedToParse
}

impl Display for ParameterError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ParameterError::NotFound => write!(f, "no parameter found"),
            ParameterError::FailedToParse => write!(f, "failed to parse parameter"),
        }
    }
}

impl Debug for ParameterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "no parameter found"),
            Self::FailedToParse => write!(f, "failed to parse parameter"),
        }
    }
}

impl std::error::Error for ParameterError {}

#[derive(Debug)]
enum section {
    None,
    Parameter,
    Type,
    Name,
    Help
}

// impl section {
//     fn to_string(&self) -> String {
//         format!("{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }
impl fmt::Display for section {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}


pub fn parse_parameter(p: &str) -> Result<Parameter,ParameterError> {
    let mut param = Parameter::new_empty();

    let mut in_section: bool = false;
    let mut in_subsection: bool = false;
    let mut in_name: bool = false;
    let mut current_section_name: String = "".to_string();
    let mut current_section: section = section::None;
    let mut section_name: String = "".to_string();
    let mut data_type_raw: String = "".to_string();

    p.replace(" ", "").replace("\n", "")
        .chars().enumerate().for_each(|c| {
            match current_section_name.to_lowercase().as_str() {
                "parameter" => {
                    current_section = section::Parameter;
                },
                _ => {},
            };
            match c.1 {
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    if in_name {
                        param.name.push(c.1);
                    }
                    if in_section {
                        current_section_name.push(c.1);
                    }
                },
                '[' => {
                    in_section = true
                },
                ']' => {
                    in_section = false
                },
                '(' => {
                    in_subsection = true
                },
                ')' => {
                    in_subsection = false
                },
                '$' => {
                    in_name = true;
                },
                '=' => {
                    if in_name {
                        in_name = false                 
                    }
                }
                _ => {}
            }

            println!("{}", c.1);
    });
    Err(ParameterError::FailedToParse)
}

#[test]
fn test_parse_parameter() {
    let p = r"[Parameter(
        Mandatory=$false
    )]
    [Boolean]
    $Boolean=$true";

    let res = parse_parameter(p);
    assert_ne!(res.is_err(), true);
}

pub struct Parameter {
    pub name: String,
    pub data_type: DATA_TYPE,
    pub default_value: bool,
    pub mandatory: bool,
    pub parameter_set_name: Option<String>,
    pub help: Option<String>,
}

impl Parameter {
    fn new_empty() -> Self {
        Parameter{
            name: "".to_string(),
            data_type: DATA_TYPE::NONE,
            default_value: false,
            mandatory: false,
            parameter_set_name: None,
            help: None,
        }
    }
    fn new(
        name: String,
        data_type: DATA_TYPE,
        default_value: bool,
        mandatory: bool,
        parameter_set_name: Option<String>,
        help: Option<String>,
    ) -> Self {
        Parameter{
            name,
            data_type,
            default_value,
            mandatory,
            parameter_set_name,
            help,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum DATA_TYPE {
    STRING,
    BOOL,
    SWITCH,
    UI32,
    UI64,
    I32,
    I64,
    F32,
    F64,
    CREDENTIAL,
    NONE,
}

static DATA_TYPES: phf::Map<&'static str, DATA_TYPE> = phf_map! {
    "string" =>  DATA_TYPE::STRING,
    "int" =>  DATA_TYPE::I32,
    "int32" =>  DATA_TYPE::I32,
    "int64" =>  DATA_TYPE::I64,
    "single" =>  DATA_TYPE::F32,
    "float" =>  DATA_TYPE::F32,
    "double" =>  DATA_TYPE::F64,
    "pscredential" =>  DATA_TYPE::CREDENTIAL,
    "" =>  DATA_TYPE::NONE,
};

#[test]
fn test_DATA_TYPES() {
    DATA_TYPES.into_iter().for_each(|(k,v)| {
        match k {
            &"string" => {
                assert_eq!(*v, DATA_TYPE::STRING)
            },
            &"int" => {
                assert_eq!(*v, DATA_TYPE::I32)
            }
            &"int32" => {
                assert_eq!(*v, DATA_TYPE::I32)
            }
            &"int64" => {
                assert_eq!(*v, DATA_TYPE::I64)
            }
            &"single" => {
                assert_eq!(*v, DATA_TYPE::F32)
            }
            &"float" => {
                assert_eq!(*v, DATA_TYPE::F32)
            }
            &"double" => {
                assert_eq!(*v, DATA_TYPE::F64)
            }
            &"pscredential" => {
                assert_eq!(*v, DATA_TYPE::CREDENTIAL)
            }
            &"" => {
                assert_eq!(*v, DATA_TYPE::NONE)
            }
            _ => {panic!("type not in map. This shouldn't have happend!")}
        }
    });
}


#[test]
fn test_new() {
    let p: Parameter = Parameter::new(
        "test".to_string(),
         DATA_TYPE::STRING,
         false,
         true,
         None,
         None,
        );
    
    assert_eq!(p.name, "test".to_string());
    assert_eq!(p.data_type, DATA_TYPE::STRING);
    assert_eq!(p.default_value, false);
    assert_eq!(p.mandatory, true);
    assert_eq!(p.parameter_set_name, None);
    assert_eq!(p.help, None);
}