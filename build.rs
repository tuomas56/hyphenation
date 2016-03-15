extern crate pocket_resources;


fn main() {
    let suffixes = vec!["pat", "hyp"];
    let langs = vec![
        "af",
        "hy",
        "as",
        "eu",
        "bn",
        "bg",
        "ca",
        "zh-latn-pinyin",
        "cop",
        "hr",
        "cs",
        "da",
        "nl",
        "en-gb",
        "en-us",
        "eo",
        "et",
        "mul-ethi",
        "fi",
        "fr",
        "fur",
        "gl",
        "ka",
        "de-1901",
        "de-1996",
        "de-ch-1901",
        "grc",
        "el-monoton",
        "el-polyton",
        "gu",
        "hi",
        "hu",
        "is",
        "id",
        "ia",
        "ga",
        "it",
        "kn",
        "kmr",
        "la",
        "la-x-classic",
        "lv",
        "lt",
        "ml",
        "mr",
        "mn-cyrl",
        "nb",
        "nn",
        "oc",
        "or",
        "pa",
        "pms",
        "pl",
        "pt",
        "ro",
        "rm",
        "ru",
        "sa",
        "sr-cyrl",
        "sh-cyrl",
        "sh-latn",
        "sk",
        "sl",
        "es",
        "sv",
        "ta",
        "te",
        "th",
        "tr",
        "tk",
        "uk",
        "hsb",
        "cy"
    ];

    let iter = langs.iter()
                    .flat_map(|tag| suffixes.iter().map(move |suffix|
                        ("patterns", format!("hyph-{}.{}.json", tag, suffix))));
    let data_paths: Vec<_> = iter.collect();

    pocket_resources::package(data_paths.iter().by_ref()).unwrap();
}