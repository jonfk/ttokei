
use super::output::Outputter;
use tokei::{Languages, LanguageType};

pub fn get_statistics<T>(outputer: &T, date: String)
    where T: Outputter
{
    // Create new Languages
    let mut languages = Languages::new();

    // Get statistics
    languages.get_statistics(vec!["."], vec![]);

    // Remove empty languages
    //let language_map = languages.remove_empty();

    outputer.output(languages);
}
