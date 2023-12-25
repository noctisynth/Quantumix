use regex::Regex;
use lazy_static::lazy_static;
pub struct EmailValidatorCacher {
    regex: &'static Regex,
}

impl EmailValidatorCacher{
    pub fn new() -> EmailValidatorCacher {
        // 使用 lazy load 减少资源开销
        lazy_static! {
            // 此处添加 多种邮箱判定
            static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@(tutanota.com|tuta.com)$")
                .expect("Invalid regex pattern");
        }
        EmailValidatorCacher {
            regex: &EMAIL_REGEX,
        }
    }

    pub fn validate(&self, email: &str) -> bool {
        self.regex.is_match(email)
    }
}

lazy_static! {
    pub static ref EMAIL_VALIDATOR: EmailValidatorCacher = EmailValidatorCacher::new();
    // 导出初始化静态 邮件检查器， 避免每次重新生成
}


#[cfg(test)]
mod tests {
    // 添加测试
    use super::EMAIL_VALIDATOR;
    
    #[test]
    fn test_email_validator() {
        let email1 = "test@tutanota.com";
        let email2 = "test@tuta.com";
        let email3 = "test@test.com";
        
        assert!(EMAIL_VALIDATOR.validate(email1));
        assert!(EMAIL_VALIDATOR.validate(email2));
        assert!(!EMAIL_VALIDATOR.validate(email3));
    }
}
