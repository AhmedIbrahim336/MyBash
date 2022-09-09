use crate::regex::*;
use regex::Regex;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VarValue {
    Int(i32),
    Str(String),
}

#[derive(Debug, Error)]
pub enum VarErr {
    #[error("`{0}` is not a valid int")]
    InvalidInt(String),
    #[error("`{0}` is not a valid variable declaration")]
    InvlaidVarDeclaration(String),
    #[error("`{0}` is not valid datatypes\nDatatype: [Int, Str, String]")]
    InvalidDataType(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variable {
    pub name: String,
    pub value: VarValue,
}

impl Variable {
    pub fn new<T: Into<String> + Display>(name: T, value: VarValue) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }

    pub fn is_var(s: &str) -> bool {
        let re = Regex::new(RE_VAR).unwrap();
        re.is_match(s)
    }
}

impl FromStr for Variable {
    type Err = VarErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(RE_VAR).unwrap();

        if let Some(caps) = re.captures(s) {
            let name = caps["name"].to_string();
            let value = caps["value"].to_string();
            let data = match &caps["type"] {
                "Str" | "String" => VarValue::Str(value),
                "Int" => match value.parse::<i32>() {
                    Ok(val) => VarValue::Int(val),
                    Err(_) => {
                        return Err(VarErr::InvalidInt(format!(
                            "`{}` is not a valid int",
                            value
                        )))
                    }
                },
                _ => return Err(VarErr::InvalidDataType(caps["type"].to_string())),
            };

            Ok(Variable { name, value: data })
        } else {
            Err(VarErr::InvlaidVarDeclaration(s.to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_new_str_var() {
        let var_name = "name".to_string();
        let var_value = "Jone".to_string();
        let var = Variable::new(&var_name, VarValue::Str(var_value.clone()));

        assert_eq!(var.name, var_name);
        assert_eq!(var.value, VarValue::Str(var_value));
    }

    #[test]
    fn create_new_int_var() {
        let var_name = "age".to_string();
        let var_value = 30;
        let var = Variable::new(&var_name, VarValue::Int(var_value));

        assert_eq!(var.name, var_name);
        assert_eq!(var.value, VarValue::Int(var_value));
    }

    #[test]
    fn new_str_var_with_double_quotes() {
        let expr = "name: Str = \"Jone\"";
        let var = expr.parse::<Variable>().unwrap();

        assert_eq!(
            var,
            Variable {
                name: "name".into(),
                value: VarValue::Str("Jone".into())
            }
        )
    }

    #[test]
    fn new_str_var_with_single_quotes() {
        let expr = "email: Str = 'something@whatmatter.com'";
        let var = expr.parse::<Variable>().unwrap();

        assert_eq!(
            var,
            Variable {
                name: "email".into(),
                value: VarValue::Str("something@whatmatter.com".into())
            }
        )
    }

    #[test]
    fn new_int_var() {
        let expr = "age: Int = 31";
        let var = expr.parse::<Variable>().unwrap();

        assert_eq!(
            var,
            Variable {
                name: "age".into(),
                value: VarValue::Int(31)
            }
        )
    }
}
