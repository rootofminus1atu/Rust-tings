use rand::seq::SliceRandom;
use rand;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PopeQuote {
    pub quote_en: String,
    pub quote_pl: String,
}

impl PopeQuote {
    fn new(quote_en: &str, quote_pl: &str) -> Self {
        Self {
            quote_en: String::from(quote_en),
            quote_pl: String::from(quote_pl),
        }
    }

    fn from_tuple((quote_en, quote_pl): (&str, &str)) -> Self {
        Self::new(quote_en, quote_pl)
    }

    fn from_list(list: Vec<(&str, &str)>) -> Vec<Self> {
        list.into_iter().map(Self::from_tuple).collect()
    }

    pub fn get_quotes() -> Vec<Self> {
        Self::from_list(vec![
            ("Yes", "Tak"),
            ("Just like Jesus said", "Tak jak pan Jezus powiedział"),
        ])
    }

    pub fn get_random_quote() -> Option<Self> {
        let quotes = Self::get_quotes();
        let mut rng = rand::thread_rng();

        quotes.choose(&mut rng).cloned()
    }
}