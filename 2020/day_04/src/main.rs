use passport_processing::*;

fn main() {
    let input = include_str!("input");
    println!("{}", solution_01(transform_input(input)));
    println!("{}", solution_02(transform_input(input)));
}

mod passport_processing {
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct Passport {
        pub pid: String,
        pub cid: Option<String>,
        pub byr: u32,
        pub iyr: u32,
        pub eyr: u32,
        pub hgt: String,
        pub hcl: String,
        pub ecl: String,
    }

    impl Passport {
        pub fn is_valid(&self) -> bool {
            let valid_byr = self.byr >= 1920 && self.byr <= 2002;
            let valid_iyr = self.iyr >= 2010 && self.iyr <= 2020;
            let valid_eyr = self.eyr >= 2020 && self.eyr <= 2030;
            let valid_hgt = if self.hgt.ends_with("cm") {
                match self.hgt.strip_suffix("cm").map(u16::from_str) {
                    Some(Ok(hgt)) if hgt >= 150 && hgt <= 193 => true,
                    _ => false,
                }
            } else if self.hgt.ends_with("in") {
                match self.hgt.strip_suffix("in").map(u16::from_str) {
                    Some(Ok(hgt)) if hgt >= 59 && hgt <= 76 => true,
                    _ => false,
                }
            } else {
                false
            };
            let valid_pid =
                self.pid.len() == 9 && self.pid.chars().filter(|c| c.is_numeric()).count() == 9;
            let valid_ecl = match self.ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            };
            let valid_hcl = self.hcl.starts_with('#')
                && if let Some(val) = self.hcl.strip_prefix('#') {
                    val.chars()
                        .filter(|c| match c {
                            '0'..='9' | 'a'..='f' => true,
                            _ => false,
                        })
                        .count()
                        == 6
                } else {
                    false
                };
                
            valid_byr && valid_eyr && valid_iyr && valid_hgt && valid_pid && valid_ecl && valid_hcl
        }
    }
    impl FromStr for Passport {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (mut pid, mut cid, mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl) =
                (None, None, None, None, None, None, None, None);
            for line in s.lines() {
                for data in line.split_ascii_whitespace() {
                    let mut data = data.trim().split(':');
                    if let (Some(key), Some(value)) = (data.next(), data.next()) {
                        match key {
                            "byr" => byr = value.parse().ok(),
                            "iyr" => iyr = value.parse().ok(),
                            "eyr" => eyr = value.parse().ok(),
                            "hgt" => hgt = value.parse().ok(),
                            "hcl" => hcl = value.parse().ok(),
                            "ecl" => ecl = value.parse().ok(),
                            "pid" => pid = value.parse().ok(),
                            "cid" => cid = value.parse().ok(),
                            _ => (),
                        }
                    }
                }
            }

            match (pid, cid, byr, iyr, eyr, hgt, hcl, ecl) {
                (
                    Some(pid),
                    cid,
                    Some(byr),
                    Some(iyr),
                    Some(eyr),
                    Some(hgt),
                    Some(hcl),
                    Some(ecl),
                ) => Ok(Passport {
                    pid,
                    cid,
                    byr,
                    iyr,
                    eyr,
                    hgt,
                    hcl,
                    ecl,
                }),
                _ => Err(()),
            }
        }
    }

    pub fn solution_01(input: Vec<Result<Passport, ()>>) -> usize {
        input.iter().flatten().count()
    }

    pub fn solution_02(input: Vec<Result<Passport, ()>>) -> usize {
        input
            .iter()
            .filter(|pass| {
                if let Ok(pass) = pass {
                    pass.is_valid()
                } else {
                    false
                }
            })
            .count()
    }

    pub fn transform_input(input: &str) -> Vec<Result<Passport, ()>> {
        input.split("\n\n").map(Passport::from_str).collect()
    }
}

#[cfg(test)]
mod test {
    use super::passport_processing::*;

    #[test]
    fn test_01() {
        let input = include_str!("example_input");
        assert_eq!(solution_01(transform_input(input)), 2);
    }

    #[test]
    fn test_02() {
        let input = include_str!("only_invalid_pass");
        assert_eq!(solution_02(transform_input(input)), 0);
    }

    #[test]
    fn test_03() {
        let input = include_str!("only_valid_pass");
        assert_eq!(solution_02(transform_input(input)), 4);
    }
}
