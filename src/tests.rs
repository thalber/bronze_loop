use crate::*;

#[test]
fn build_sample_lookalike() {
    let lookalikes_string = "iıɩɪⅰｉιіӏᎥιℹⅈ𝐢𝑖𝒊𝒾𝓲𝔦𝕚𝖎𝗂𝗶𝘪𝙞𝚒𝚤𝛊𝜄𝜾𝝸𝞲⍳";
    let lookalikes = crate::Lookalikes::new(lookalikes_string, |c| match c {
        'i' => Some(LocaleID::En),
        _ => None,
    })
    .unwrap();
    let test_chars = vec!['ı', 'ɪ', 'ɪ'];
    let filtered_chars: Vec<_> = test_chars.into_iter().filter_map(|c| lookalikes.try_convert_to_locale(&c, LocaleID::En).ok()).collect();
    let desired_chars = vec!['i', 'i', 'i'];
    assert_eq!(filtered_chars, desired_chars)
}

#[test]
fn normalize_text() {
    let start_text = "iıɩɪⅰｉιіӏᎥ";
    let lookalikes = crate::Lookalikes::new(start_text, |c| match c {
        'i' => Some(LocaleID::En),
        _ => None,
    })
    .unwrap();
    let locales = Locale::create_defaults();
    let lookalike_sets = vec![lookalikes];
    let tnr = make_normalized_text(start_text, &locales, &lookalike_sets, LocaleID::En);
    let (normalized, obfuscation_detected) = (tnr.value, tnr.lookalikes_removed);
    assert_eq!(normalized.as_str(), "iiiiiiiiii")
}
