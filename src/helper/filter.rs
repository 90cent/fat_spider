use regex::Regex;

pub mod filter_presets {
    pub struct Filter {
        pub input: String,
        pub pattern: String,
        pub matches: Vec<String>
    }
    
    #[allow(dead_code)]
    pub enum Presets {
        EmailPassword,      // Maxistdumm@gmail.com:UndSeinPasswordIst123
        EMAIL,      // Maxisteinhs@gmail.com
        IpPort,      // 127.0.0.1:5091 can be used for proxys
        DiscordToken,
        DiscordTokenMfa,
        None
    }

    pub trait PresetTrait {
        fn get_pattern(&self) -> String; 
        fn create_filter(&self,input: String) -> Filter;
    }

    impl PresetTrait for Presets {
        fn get_pattern(&self) -> String
        {
            match &self {
                Presets::EmailPassword => String::from(r"([A-z0-9@]{3,256}[.{a-z}]{2,4})([:]\S{0,256})"),
                Presets::EMAIL => String::from(r"([A-z0-9@]{3,256}[.{a-z}]{2,4})"),
                Presets::IpPort => String::from(r"([0-9.]{8,21})([:0-9]{1,5})"),
                Presets::DiscordToken => String::from(r"^([A-z0-9\W]{52})$"),
                Presets::DiscordTokenMfa => String::from(r"^([mfa.A-z0-9\W]{81,88})$"),
                Presets::None => String::from(""),
            }
        }

        fn create_filter(&self,input: String) -> Filter
        {
            let i= input.to_owned();
            let f: Filter = Filter {
                input: i,
                pattern: self.get_pattern(),
                matches: vec![String::new()],
            };
            return f;
        }
    }
}

pub fn filter_links(rawhtml_content: Vec<String>) -> Vec<String> { // Scans every page of the Websites in the Vector
    let mut links = Vec::new();

    let links = std::thread::spawn(move || {
        let regex = Regex::new(r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,4}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*[^'&amp;'])").unwrap();

        for page in rawhtml_content {
            println!("{}",page);
            let capture = regex.find_iter(page.as_str());
                for link in capture {
                    links.push(link.as_str().into());
                }
        }
        println!("{:#?}",links);

        return links;
    }).join().unwrap();

    return links;
}

pub fn filter_context(filter: &mut filter_presets::Filter) {
    let regex = Regex::new(&filter.pattern.as_str()).unwrap();

    let capture = regex.find_iter(&filter.input.as_str());
    for link in capture {
        filter.matches.push(link.as_str().into());
    }
    println!("{:#?}",&filter.matches);
}