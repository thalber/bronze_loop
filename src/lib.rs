#[cfg(test)]
mod tests;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum LocaleID {
    Ru,
    En,
}

#[derive(Debug)]
pub enum Error {
    LookalikesMissingSymbol(char),
    LookalikesMissingLocale(LocaleID),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)?;
        //todo: add proper prettyprint?
        Ok(())
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq)]
pub struct Locale {
    pub symbols: Vec<char>,
    pub id: LocaleID,
}

#[derive(Debug)]
pub struct Lookalikes {
    pub all_symbols: Vec<char>,
    pub by_locale: cycle_map::CycleMap<LocaleID, char>,
}

#[derive(Debug)]
pub struct TextNormalizationResult {
    pub lookalikes_removed: bool,
    pub value: String,
}

impl Locale {
    pub fn new(text: &str, id: LocaleID) -> Result<Self, Error> {
        let mut symbols: Vec<char> = text.chars().collect();
        symbols.sort();
        Ok(Locale { symbols, id })
    }

    pub fn create_defaults() -> Vec<Locale> {
        vec![
            Locale::new(
                "АаБбВвГгДдЕеЁёЖжЗзИиЙйКкЛлМмНнОоПпСсТтУуФфХхЦцЧчШшЩщЪъЫыЬьЭэЮюЯя",
                LocaleID::Ru,
            )
            .unwrap(),
            Locale::new(
                "abcdefeghjiklmnopqrstuvwxyzABCDEFEGHJIKLMNOPQRSTUVWXYZ",
                LocaleID::En,
            )
            .unwrap(),
        ]
    }
}

impl Lookalikes {
    pub fn contains_symbol(&self, char: &char) -> bool {
        self.all_symbols.binary_search(char).is_ok()
    }
    pub fn try_convert_to_locale(&self, char: &char, locale: LocaleID) -> Result<char, Error> {
        if !self.contains_symbol(char) {
            return Err(Error::LookalikesMissingSymbol(*char));
        }
        return match self.by_locale.get_right(&locale) {
            Some(out_char) => Ok(*out_char),
            None => Err(Error::LookalikesMissingLocale(locale)),
        };
    }
    pub fn new(
        all_symbols: &str,
        symbol_sorter: impl Fn(&char) -> Option<LocaleID>,
    ) -> Result<Lookalikes, Error> {
        let mut all_symbols: Vec<char> = all_symbols.chars().into_iter().collect();
        all_symbols.sort();
        let by_locale = all_symbols
            .iter()
            .filter_map(|c| symbol_sorter(c).map(|loc| (loc, *c)))
            .collect();
        Ok(Lookalikes {
            all_symbols,
            by_locale,
        })
    }
}

pub fn make_normalized_text(
    text: &str,
    _locales: &Vec<Locale>,
    lookalike_sets: &Vec<Lookalikes>,
    target_locale: LocaleID,
) -> TextNormalizationResult {
    let newchars = text.chars().into_iter().map(|c| {
        let mut selected_replacement = lookalike_sets
            .iter()
            .filter_map(|set| set.try_convert_to_locale(&c, target_locale).ok());
        match selected_replacement.next() {
            Some(newchar) => newchar,
            None => c,
        }
    });
    let res: String = newchars.collect();
    TextNormalizationResult {
        lookalikes_removed: res.as_str() != text,
        value: res,
    }
    //(res.as_str() != text, res)
}


