use password_phylosophy::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod password_phylosophy {
    use std::str::FromStr;

    pub struct PasswordInfo {
        pub policy: (u16, u16, char),
        pub password: String,
        pub is_valid_policy_01: bool,
        pub is_valid_policy_02: bool
    }

    impl PasswordInfo {
        pub fn new(policy: (u16, u16, char), password: String) -> Self {
            Self {
                is_valid_policy_01: Self::is_valid_policy_01(policy, &password),
                is_valid_policy_02: Self::is_valid_policy_02(policy, &password),
                policy,
                password,
            }
        }

        fn is_valid_policy_01(policy: (u16, u16, char), password: &str) -> bool {
            let mut appeared = 0;
            for c in password.chars() {
                if c == policy.2 {
                    appeared += 1;
                }
            }
            appeared >= policy.0 && appeared <= policy.1
        }

        fn is_valid_policy_02(policy: (u16, u16, char), password: &str) -> bool {
            let password = password.as_bytes();
            match (password[(policy.0 - 1) as usize] as char, password[(policy.1 - 1) as usize] as char) {
                (i, j) if i == policy.2 && j == policy.2 => false,
                (i, _) if i == policy.2 => true,
                (_, j) if j == policy.2 => true,
                _ => false
            }
        }
    }
    

    impl FromStr for PasswordInfo {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut args = s.split(':');
            if let Some(policy) = args.next() {
                let mut policy_spec = policy.split(' ');
                if let Some(range) = policy_spec.next() {
                    let mut range = range.split('-');
                    if let (Some(min), Some(max)) = (range.next(), range.next()) {
                        if let (Ok(min), Ok(max), Some(Some(character)), Some(password)) = (
                            min.parse(),
                            max.parse(),
                            policy_spec.next().map(|c| c.chars().next()),
                            args.next(),
                        ) {
                            return Ok(PasswordInfo::new((min, max, character), password.trim().into()));
                        }
                    }
                }
            }
            Err(())
        }
    }

    pub fn solution_01(input: Vec<PasswordInfo>) -> usize {
        input.iter().filter(|info| info.is_valid_policy_01).count()
    }

    pub fn solution_02(input: Vec<PasswordInfo>) -> usize {
        input.iter().filter(|info| info.is_valid_policy_02).count()
    }

    pub fn transform_input(input: &str) -> Vec<PasswordInfo> {
        input
            .lines()
            .map(|line| line.parse::<PasswordInfo>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::password_phylosophy::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 2);
    }

    #[test]
    fn test_02() {
        let input = include_str!("example_input");
        assert_eq!(solution_02(transform_input(input)), 1);
    }
}
