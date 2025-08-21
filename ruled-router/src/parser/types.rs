//! 类型转换实现
//!
//! 为基本类型实现 FromParam 和 ToParam trait

use crate::error::ParseError;
use crate::traits::{FromParam, ToParam};

/// 为基本数字类型实现 FromParam 和 ToParam
macro_rules! impl_from_to_param_for_numbers {
    ($($t:ty),*) => {
        $(
            impl FromParam for $t {
                fn from_param(param: &str) -> Result<Self, ParseError> {
                    param.parse().map_err(|_| {
                        ParseError::type_conversion(format!(
                            "Cannot convert '{}' to {}", param, stringify!($t)
                        ))
                    })
                }
            }

            impl ToParam for $t {
                fn to_param(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

// 实现所有基本数字类型
impl_from_to_param_for_numbers!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

/// String 的实现
impl FromParam for String {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    Ok(param.to_string())
  }
}

impl ToParam for String {
  fn to_param(&self) -> String {
    self.clone()
  }
}

/// &str 的实现
impl ToParam for &str {
  fn to_param(&self) -> String {
    self.to_string()
  }
}

/// bool 的实现
impl FromParam for bool {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    match param.to_lowercase().as_str() {
      "true" | "1" | "yes" | "on" => Ok(true),
      "false" | "0" | "no" | "off" => Ok(false),
      _ => Err(ParseError::type_conversion(format!(
        "Cannot convert '{param}' to bool. Valid values: true/false, 1/0, yes/no, on/off"
      ))),
    }
  }
}

impl ToParam for bool {
  fn to_param(&self) -> String {
    self.to_string()
  }
}

/// Option<T> 的实现
impl<T: FromParam> FromParam for Option<T> {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    if param.is_empty() {
      Ok(None)
    } else {
      T::from_param(param).map(Some)
    }
  }
}

impl<T: ToParam> ToParam for Option<T> {
  fn to_param(&self) -> String {
    match self {
      Some(value) => value.to_param(),
      None => String::new(),
    }
  }
}

/// Vec<T> 的实现（用于多值参数）
impl<T: FromParam> FromParam for Vec<T> {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    if param.is_empty() {
      Ok(Vec::new())
    } else {
      // 假设多个值用逗号分隔
      param.split(',').map(|s| T::from_param(s.trim())).collect()
    }
  }
}

impl<T: ToParam> ToParam for Vec<T> {
  fn to_param(&self) -> String {
    self.iter().map(|item| item.to_param()).collect::<Vec<_>>().join(",")
  }
}

/// char 的实现
impl FromParam for char {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    let mut chars = param.chars();
    match (chars.next(), chars.next()) {
      (Some(c), None) => Ok(c),
      _ => Err(ParseError::type_conversion(format!(
        "Cannot convert '{param}' to char. Expected exactly one character"
      ))),
    }
  }
}

impl ToParam for char {
  fn to_param(&self) -> String {
    self.to_string()
  }
}

/// 自定义类型示例：UserId
///
/// 展示如何为自定义类型实现 FromParam 和 ToParam
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub u32);

impl FromParam for UserId {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    u32::from_param(param).map(UserId)
  }
}

impl ToParam for UserId {
  fn to_param(&self) -> String {
    self.0.to_string()
  }
}

/// 自定义类型示例：Slug
///
/// URL 友好的字符串标识符
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slug(pub String);

impl FromParam for Slug {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    // 验证 slug 格式：只允许字母、数字、连字符和下划线
    if param.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
      Ok(Slug(param.to_string()))
    } else {
      Err(ParseError::type_conversion(format!(
        "Invalid slug '{param}'. Slugs can only contain letters, numbers, hyphens, and underscores"
      )))
    }
  }
}

impl ToParam for Slug {
  fn to_param(&self) -> String {
    self.0.clone()
  }
}

/// 自定义类型示例：Email
///
/// 简单的邮箱地址验证
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl FromParam for Email {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    // 简单的邮箱验证：包含 @ 符号且两边都有内容
    if param.contains('@') && param.split('@').count() == 2 {
      let parts: Vec<&str> = param.split('@').collect();
      if !parts[0].is_empty() && !parts[1].is_empty() && parts[1].contains('.') {
        return Ok(Email(param.to_string()));
      }
    }
    Err(ParseError::type_conversion(format!("Invalid email address: {param}")))
  }
}

impl ToParam for Email {
  fn to_param(&self) -> String {
    self.0.clone()
  }
}

/// 自定义类型示例：Version
///
/// 语义化版本号
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
  pub major: u32,
  pub minor: u32,
  pub patch: u32,
}

impl FromParam for Version {
  fn from_param(param: &str) -> Result<Self, ParseError> {
    let parts: Vec<&str> = param.split('.').collect();
    if parts.len() != 3 {
      return Err(ParseError::type_conversion(format!(
        "Invalid version format '{param}'. Expected format: major.minor.patch"
      )));
    }

    let major = parts[0]
      .parse()
      .map_err(|_| ParseError::type_conversion(format!("Invalid major version: {}", parts[0])))?;

    let minor = parts[1]
      .parse()
      .map_err(|_| ParseError::type_conversion(format!("Invalid minor version: {}", parts[1])))?;

    let patch = parts[2]
      .parse()
      .map_err(|_| ParseError::type_conversion(format!("Invalid patch version: {}", parts[2])))?;

    Ok(Version { major, minor, patch })
  }
}

impl ToParam for Version {
  fn to_param(&self) -> String {
    format!("{}.{}.{}", self.major, self.minor, self.patch)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_number_conversions() {
    assert_eq!(u32::from_param("123").unwrap(), 123);
    assert_eq!(i32::from_param("-456").unwrap(), -456);
    assert_eq!(f64::from_param(&std::f64::consts::PI.to_string()).unwrap(), std::f64::consts::PI);

    assert!(u32::from_param("abc").is_err());
    assert!(i32::from_param("12.34").is_err());

    assert_eq!(123u32.to_param(), "123");
    assert_eq!((-456i32).to_param(), "-456");
    assert_eq!(std::f64::consts::PI.to_param(), "3.141592653589793");
  }

  #[test]
  fn test_string_conversions() {
    assert_eq!(String::from_param("hello").unwrap(), "hello");
    assert_eq!("world".to_param(), "world");
    assert_eq!("test".to_string().to_param(), "test");
  }

  #[test]
  fn test_bool_conversions() {
    assert!(bool::from_param("true").unwrap());
    assert!(!bool::from_param("false").unwrap());
    assert!(bool::from_param("1").unwrap());
    assert!(!bool::from_param("0").unwrap());
    assert!(bool::from_param("yes").unwrap());
    assert!(!bool::from_param("no").unwrap());
    assert!(bool::from_param("on").unwrap());
    assert!(!bool::from_param("off").unwrap());

    assert!(bool::from_param("maybe").is_err());

    assert_eq!(true.to_param(), "true");
    assert_eq!(false.to_param(), "false");
  }

  #[test]
  fn test_option_conversions() {
    assert_eq!(Option::<u32>::from_param("").unwrap(), None);
    assert_eq!(Option::<u32>::from_param("123").unwrap(), Some(123));

    assert_eq!(Some(123u32).to_param(), "123");
    assert_eq!(None::<u32>.to_param(), "");
  }

  #[test]
  fn test_vec_conversions() {
    assert_eq!(Vec::<u32>::from_param("").unwrap(), Vec::<u32>::new());
    assert_eq!(Vec::<u32>::from_param("1,2,3").unwrap(), vec![1, 2, 3]);
    assert_eq!(Vec::<String>::from_param("a,b,c").unwrap(), vec!["a", "b", "c"]);

    assert_eq!(vec![1u32, 2, 3].to_param(), "1,2,3");
    assert_eq!(vec!["a".to_string(), "b".to_string()].to_param(), "a,b");
  }

  #[test]
  fn test_char_conversions() {
    assert_eq!(char::from_param("a").unwrap(), 'a');
    assert_eq!(char::from_param("中").unwrap(), '中');

    assert!(char::from_param("").is_err());
    assert!(char::from_param("ab").is_err());

    assert_eq!('x'.to_param(), "x");
  }

  #[test]
  fn test_user_id() {
    assert_eq!(UserId::from_param("123").unwrap(), UserId(123));
    assert!(UserId::from_param("abc").is_err());

    assert_eq!(UserId(456).to_param(), "456");
  }

  #[test]
  fn test_slug() {
    assert_eq!(Slug::from_param("hello-world_123").unwrap(), Slug("hello-world_123".to_string()));
    assert!(Slug::from_param("hello world").is_err()); // 空格不允许
    assert!(Slug::from_param("hello@world").is_err()); // @ 不允许

    assert_eq!(Slug("test-slug".to_string()).to_param(), "test-slug");
  }

  #[test]
  fn test_email() {
    assert_eq!(
      Email::from_param("user@example.com").unwrap(),
      Email("user@example.com".to_string())
    );
    assert!(Email::from_param("invalid-email").is_err());
    assert!(Email::from_param("@example.com").is_err());
    assert!(Email::from_param("user@").is_err());

    assert_eq!(Email("test@example.com".to_string()).to_param(), "test@example.com");
  }

  #[test]
  fn test_version() {
    assert_eq!(
      Version::from_param("1.2.3").unwrap(),
      Version {
        major: 1,
        minor: 2,
        patch: 3
      }
    );
    assert!(Version::from_param("1.2").is_err());
    assert!(Version::from_param("1.2.3.4").is_err());
    assert!(Version::from_param("a.b.c").is_err());

    assert_eq!(
      Version {
        major: 2,
        minor: 1,
        patch: 0
      }
      .to_param(),
      "2.1.0"
    );
  }
}
