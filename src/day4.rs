use regex::Regex;

pub fn solve_floor() {
    let data = include_str!("../resources/4-1.txt");
    let passports: Vec<Passport> = data.split("\n\n")
                                       .map(|entry| construct_passport(entry)).collect();

    println!("Valid Passports: {}", passports.iter().filter(|passport| validate_passport(*passport)).count());
}

pub fn solve_basement() {
    let data = include_str!("../resources/4-1.txt");
    let passports: Vec<Passport> = data.split("\n\n")
                                       .map(|entry| construct_passport(entry)).collect();

    println!("Valid Passports: {}", passports.iter().filter(|passport| validate_passport_strict(*passport)).count());
}

fn construct_passport(entry: &str) -> Passport {
    let mut passport: Passport = Passport {
        birth_year: "",
        issue_year: "",
        expiration_year: "",
        height: "",
        hair_color: "",
        eye_color: "",
        passport_id: "",
        country_id: "",
    };

    entry.split_whitespace().for_each(|item| {
        let mut split = item.split(':');
        let key = split.next().unwrap();
        let value = split.next().unwrap();

        assert_eq!(split.count(), 0, "Passport entry was formatted weirdly!");

        match key {
            "byr" => passport.birth_year = value,
            "iyr" => passport.issue_year = value,
            "eyr" => passport.expiration_year = value,
            "hgt" => passport.height = value,
            "hcl" => passport.hair_color = value,
            "ecl" => passport.eye_color = value,
            "pid" => passport.passport_id = value,
            "cid" => passport.country_id = value,
            _ => { panic!("Found a passport entry that we don't know about!") }
        }
    });

    passport
}

fn validate_passport(passport: &Passport) -> bool {
    return !passport.birth_year.is_empty()
        && !passport.issue_year.is_empty()
        && !passport.expiration_year.is_empty()
        && !passport.height.is_empty()
        && !passport.hair_color.is_empty()
        && !passport.eye_color.is_empty()
        && !passport.passport_id.is_empty();
}

fn validate_passport_strict(passport: &Passport) -> bool {
    if !validate_passport(passport) {
        return false;
    }

    let birth_year_pass = match passport.birth_year.parse::<i32>() {
        Ok(num) => num >= 1920 && num <= 2020,
        Err(_) => false
    };

    let issue_year_pass = match passport.issue_year.parse::<i32>() {
        Ok(num) => num >= 2010 && num <= 2020,
        Err(_) => false
    };

    let expiration_year_pass = match passport.expiration_year.parse::<i32>() {
        Ok(num) => num >= 2020 && num <= 2030,
        Err(_) => false
    };

    let len = passport.height.len();
    let height_pass = if passport.height.ends_with("cm") {
        match passport.height[..len - 2].parse::<i32>() {
            Ok(num) => num >= 150 && num <= 193,
            Err(_) => false
        }
    } else if passport.height.ends_with("in") {
        match passport.height[..len - 2].parse::<i32>() {
            Ok(num) => num >= 59 && num <= 76,
            Err(_) => false
        }
    } else {
        false
    };

    // A macro that ensures the regex is only compiled once.
    lazy_static! {
        static ref HAIR_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    }
    let hair_pass = HAIR_REGEX.is_match(passport.hair_color);

    let eye_pass = match passport.eye_color {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false
    };

    let passport_num_pass = passport.passport_id.len() == 9 && passport.passport_id.parse::<i32>().is_ok();


    birth_year_pass
        && issue_year_pass
        && expiration_year_pass
        && height_pass
        && hair_pass
        && eye_pass
        && passport_num_pass
}

struct Passport<'a> {
    birth_year: &'a str,
    issue_year: &'a str,
    expiration_year: &'a str,
    height: &'a str,
    hair_color: &'a str,
    eye_color: &'a str,
    passport_id: &'a str,
    country_id: &'a str,
}
