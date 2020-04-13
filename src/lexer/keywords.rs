use crate::lexer::tokens::TokenType;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Keywords {
    pub list: HashMap<String, TokenType>,
}

impl Keywords {
    pub fn new() -> Self {
        let mut list = HashMap::new();

        // Vec<str>s contain keywords that are named meme_style
        // Last word will be in മലയാളലിപി(Malayalalipi)
        for pwoli_sanam in vec![
            "pwoli_sadhanam",
            "pwoli_sanam",
            "pwoli_saanam",
            "poli_sadhanam",
            "poli_sanam",
            "poli_saanam",
            "പൊളിസാധനം",
        ] {
            list.insert(pwoli_sanam.to_string(), TokenType::Declaration);
        }

        for addr_tha in vec!["address_thada", "അഡ്രസ്_താടാ"] {
            list.insert(addr_tha.to_string(), TokenType::InputString);
        }

        for num_tha in vec!["number_thada", "നമ്പർ_താടാ"] {
            list.insert(num_tha.to_string(), TokenType::InputNumber);
        }

        for da_pidi in vec!["dhe_pidicho", "ദേ_പിടിച്ചോ"] {
            list.insert(da_pidi.to_string(), TokenType::Write);
        }

        for seriano_mone in vec!["seriyano_mwone", "seriyano", "ശെരിയാണോ_മോനെ"]
        {
            list.insert(seriano_mone.to_string(), TokenType::If);
        }

        for seri_alle in vec!["seri_allel", "ശെരി_അല്ലേൽ"] {
            list.insert(seri_alle.to_string(), TokenType::Else);
        }

        for repeat_adi in vec!["repeat_adi", "റിപീറ്റടി"] {
            list.insert(repeat_adi.to_string(), TokenType::Loop);
        }

        for veluth_aan in vec!["veluthane", "വലുതാണെ"] {
            list.insert(veluth_aan.to_string(), TokenType::GreaterThan);
        }

        for same_alle in vec!["same_alle", "same_allel", "സെയിം_അല്ല"] {
            list.insert(same_alle.to_string(), TokenType::NotEqual);
        }

        for cheruth_aane in vec!["cheruthane", "ചെറുതാണെ"] {
            list.insert(cheruth_aane.to_string(), TokenType::LessThan);
        }

        for same_aane in vec!["same_aane", "സെയിം_ആണേ"] {
            list.insert(same_aane.to_string(), TokenType::EqualTo);
        }

        for um in vec!["um", "ഉം"] {
            list.insert(um.to_string(), TokenType::Um);
        }

        for ne_kal in vec!["ne_kal", "നെകാൾ"] {
            list.insert(ne_kal.to_string(), TokenType::Nekal);
        }

        Self { list }
    }
}
