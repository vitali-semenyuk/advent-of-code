use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
struct Passport {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    birth_year: u16,
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    issue_year: u16,
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    expiration_year: u16,
    // hgt (Height) - a number followed by either cm or in:
    height: String,
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    hair_color: String,
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    eye_color: String,
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    passport_id: String,
    _country_id: Option<u16>,
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

impl Passport {
    fn is_valid(&self) -> bool {
        let is_height_valid = if self.height.ends_with("cm") {
            self.height.len() == 5 && self.height[..3].parse::<u8>().is_ok()
        } else if self.height.ends_with("in") {
            self.height.len() == 4 && self.height[..2].parse::<u8>().is_ok()
        } else {
            false
        };

        (1920..=2002).contains(&self.birth_year)
            && (2010..=2020).contains(&self.issue_year)
            && (2020..=2030).contains(&self.expiration_year)
            && is_height_valid
            && self.hair_color.len() == 7
            && self.hair_color.starts_with('#')
            && self.hair_color[1..].chars().all(|c| c.is_ascii_hexdigit())
            && EYE_COLORS.contains(&self.eye_color.as_str())
            && self.passport_id.len() == 9
            && self.passport_id.chars().all(|c| c.is_alphanumeric())
    }
}

#[derive(Debug)]
struct PassportParseError;

impl TryFrom<&str> for Passport {
    type Error = PassportParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fields_map = value
            .split_whitespace()
            .fold(HashMap::new(), |mut fields, data| {
                let (field, value) = data.split_once(':').unwrap();
                fields.insert(field, value);
                fields
            });

        let birth_year = fields_map
            .get("byr")
            .map(|v| v.parse::<u16>())
            .ok_or(PassportParseError)?
            .map_err(|_| PassportParseError)?;
        let issue_year = fields_map
            .get("iyr")
            .map(|v| v.parse::<u16>())
            .ok_or(PassportParseError)?
            .map_err(|_| PassportParseError)?;
        let expiration_year = fields_map
            .get("eyr")
            .map(|v| v.parse::<u16>())
            .ok_or(PassportParseError)?
            .map_err(|_| PassportParseError)?;
        let height = fields_map.get("hgt").ok_or(PassportParseError)?.to_string();
        let hair_color = fields_map.get("hcl").ok_or(PassportParseError)?.to_string();
        let eye_color = fields_map.get("ecl").ok_or(PassportParseError)?.to_string();
        let passport_id = fields_map.get("pid").ok_or(PassportParseError)?.to_string();
        let country_id = fields_map
            .get("cid")
            .map(|v| v.parse::<u16>())
            .transpose()
            .map_err(|_| PassportParseError)?;

        Ok(Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            _country_id: country_id,
        })
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::try_from)
        .filter_map(|r| r.ok())
        .count()
}

fn solve_second_part(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::try_from)
        .filter_map(|r| r.ok())
        .filter(|p| p.is_valid())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let answer = 2;

        assert_eq!(answer, solve_first_part(input))
    }

    #[test]
    fn test_second_part_invalid() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let answer = 0;

        assert_eq!(answer, solve_second_part(input))
    }

    #[test]
    fn test_second_part_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let answer = 4;

        assert_eq!(answer, solve_second_part(input))
    }

    check_answers!(260, 153);
}
