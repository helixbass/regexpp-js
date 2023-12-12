use std::{collections::HashSet, cell::OnceCell};

use once_cell::sync::Lazy;

use crate::ecma_versions::EcmaVersion;

#[derive(Clone, Debug)]
struct DataSet {
    _raw2018: &'static str,
    _raw2019: &'static str,
    _raw2020: &'static str,
    _raw2021: &'static str,
    _raw2022: &'static str,
    _raw2023: &'static str,
    _raw2024: &'static str,
    _set2018: OnceCell<HashSet<&'static str>>,
    _set2019: OnceCell<HashSet<&'static str>>,
    _set2020: OnceCell<HashSet<&'static str>>,
    _set2021: OnceCell<HashSet<&'static str>>,
    _set2022: OnceCell<HashSet<&'static str>>,
    _set2023: OnceCell<HashSet<&'static str>>,
    _set2024: OnceCell<HashSet<&'static str>>,
}

impl DataSet {
    pub fn new(
        raw2018: &'static str,
        raw2019: &'static str,
        raw2020: &'static str,
        raw2021: &'static str,
        raw2022: &'static str,
        raw2023: &'static str,
        raw2024: &'static str,
    ) -> Self {
        Self {
            _raw2018: raw2018,
            _raw2019: raw2019,
            _raw2020: raw2020,
            _raw2021: raw2021,
            _raw2022: raw2022,
            _raw2023: raw2023,
            _raw2024: raw2024,
            _set2018: Default::default(),
            _set2019: Default::default(),
            _set2020: Default::default(),
            _set2021: Default::default(),
            _set2022: Default::default(),
            _set2023: Default::default(),
            _set2024: Default::default(),
        }
    }

    pub fn es2018(&self) -> &HashSet<&'static str> {
        self._set2018.get_or_init(|| {
            self._raw2018.split(' ').collect()
        })
    }

    pub fn es2019(&self) -> &HashSet<&'static str> {
        self._set2019.get_or_init(|| {
            self._raw2019.split(' ').collect()
        })
    }

    pub fn es2020(&self) -> &HashSet<&'static str> {
        self._set2020.get_or_init(|| {
            self._raw2020.split(' ').collect()
        })
    }

    pub fn es2021(&self) -> &HashSet<&'static str> {
        self._set2021.get_or_init(|| {
            self._raw2021.split(' ').collect()
        })
    }

    pub fn es2022(&self) -> &HashSet<&'static str> {
        self._set2022.get_or_init(|| {
            self._raw2022.split(' ').collect()
        })
    }

    pub fn es2023(&self) -> &HashSet<&'static str> {
        self._set2023.get_or_init(|| {
            self._raw2023.split(' ').collect()
        })
    }

    pub fn es2024(&self) -> &HashSet<&'static str> {
        self._set2024.get_or_init(|| {
            self._raw2024.split(' ').collect()
        })
    }
}

static GC_NAME_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    ["General_Category", "gc"].into_iter().collect()
});

static SC_NAME_SET: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    ["Script", "Script_Extensions", "sc", "scx"].into_iter().collect()
});

thread_local! {
    static GC_VALUE_SETS: once_cell::unsync::Lazy<DataSet> = once_cell::unsync::Lazy::new(|| {
        DataSet::new(
            "C Cased_Letter Cc Cf Close_Punctuation Cn Co Combining_Mark Connector_Punctuation Control Cs Currency_Symbol Dash_Punctuation Decimal_Number Enclosing_Mark Final_Punctuation Format Initial_Punctuation L LC Letter Letter_Number Line_Separator Ll Lm Lo Lowercase_Letter Lt Lu M Mark Math_Symbol Mc Me Mn Modifier_Letter Modifier_Symbol N Nd Nl No Nonspacing_Mark Number Open_Punctuation Other Other_Letter Other_Number Other_Punctuation Other_Symbol P Paragraph_Separator Pc Pd Pe Pf Pi Po Private_Use Ps Punctuation S Sc Separator Sk Sm So Space_Separator Spacing_Mark Surrogate Symbol Titlecase_Letter Unassigned Uppercase_Letter Z Zl Zp Zs cntrl digit punct",
            "",
            "",
            "",
            "",
            "",
            "",
        )
    });

    static SC_VALUE_SETS: once_cell::unsync::Lazy<DataSet> = once_cell::unsync::Lazy::new(|| {
        DataSet::new(
            "Adlam Adlm Aghb Ahom Anatolian_Hieroglyphs Arab Arabic Armenian Armi Armn Avestan Avst Bali Balinese Bamu Bamum Bass Bassa_Vah Batak Batk Beng Bengali Bhaiksuki Bhks Bopo Bopomofo Brah Brahmi Brai Braille Bugi Buginese Buhd Buhid Cakm Canadian_Aboriginal Cans Cari Carian Caucasian_Albanian Chakma Cham Cher Cherokee Common Copt Coptic Cprt Cuneiform Cypriot Cyrillic Cyrl Deseret Deva Devanagari Dsrt Dupl Duployan Egyp Egyptian_Hieroglyphs Elba Elbasan Ethi Ethiopic Geor Georgian Glag Glagolitic Gonm Goth Gothic Gran Grantha Greek Grek Gujarati Gujr Gurmukhi Guru Han Hang Hangul Hani Hano Hanunoo Hatr Hatran Hebr Hebrew Hira Hiragana Hluw Hmng Hung Imperial_Aramaic Inherited Inscriptional_Pahlavi Inscriptional_Parthian Ital Java Javanese Kaithi Kali Kana Kannada Katakana Kayah_Li Khar Kharoshthi Khmer Khmr Khoj Khojki Khudawadi Knda Kthi Lana Lao Laoo Latin Latn Lepc Lepcha Limb Limbu Lina Linb Linear_A Linear_B Lisu Lyci Lycian Lydi Lydian Mahajani Mahj Malayalam Mand Mandaic Mani Manichaean Marc Marchen Masaram_Gondi Meetei_Mayek Mend Mende_Kikakui Merc Mero Meroitic_Cursive Meroitic_Hieroglyphs Miao Mlym Modi Mong Mongolian Mro Mroo Mtei Mult Multani Myanmar Mymr Nabataean Narb Nbat New_Tai_Lue Newa Nko Nkoo Nshu Nushu Ogam Ogham Ol_Chiki Olck Old_Hungarian Old_Italic Old_North_Arabian Old_Permic Old_Persian Old_South_Arabian Old_Turkic Oriya Orkh Orya Osage Osge Osma Osmanya Pahawh_Hmong Palm Palmyrene Pau_Cin_Hau Pauc Perm Phag Phags_Pa Phli Phlp Phnx Phoenician Plrd Prti Psalter_Pahlavi Qaac Qaai Rejang Rjng Runic Runr Samaritan Samr Sarb Saur Saurashtra Sgnw Sharada Shavian Shaw Shrd Sidd Siddham SignWriting Sind Sinh Sinhala Sora Sora_Sompeng Soyo Soyombo Sund Sundanese Sylo Syloti_Nagri Syrc Syriac Tagalog Tagb Tagbanwa Tai_Le Tai_Tham Tai_Viet Takr Takri Tale Talu Tamil Taml Tang Tangut Tavt Telu Telugu Tfng Tglg Thaa Thaana Thai Tibetan Tibt Tifinagh Tirh Tirhuta Ugar Ugaritic Vai Vaii Wara Warang_Citi Xpeo Xsux Yi Yiii Zanabazar_Square Zanb Zinh Zyyy",
            "Dogr Dogra Gong Gunjala_Gondi Hanifi_Rohingya Maka Makasar Medefaidrin Medf Old_Sogdian Rohg Sogd Sogdian Sogo",
            "Elym Elymaic Hmnp Nand Nandinagari Nyiakeng_Puachue_Hmong Wancho Wcho",
            "Chorasmian Chrs Diak Dives_Akuru Khitan_Small_Script Kits Yezi Yezidi",
            "Cpmn Cypro_Minoan Old_Uyghur Ougr Tangsa Tnsa Toto Vith Vithkuqi",
            "Hrkt Katakana_Or_Hiragana Kawi Nag_Mundari Nagm Unknown Zzzz",
            "",
        )
    });

    static BIN_PROPERTY_SETS: once_cell::unsync::Lazy<DataSet> = once_cell::unsync::Lazy::new(|| {
        DataSet::new(
            "AHex ASCII ASCII_Hex_Digit Alpha Alphabetic Any Assigned Bidi_C Bidi_Control Bidi_M Bidi_Mirrored CI CWCF CWCM CWKCF CWL CWT CWU Case_Ignorable Cased Changes_When_Casefolded Changes_When_Casemapped Changes_When_Lowercased Changes_When_NFKC_Casefolded Changes_When_Titlecased Changes_When_Uppercased DI Dash Default_Ignorable_Code_Point Dep Deprecated Dia Diacritic Emoji Emoji_Component Emoji_Modifier Emoji_Modifier_Base Emoji_Presentation Ext Extender Gr_Base Gr_Ext Grapheme_Base Grapheme_Extend Hex Hex_Digit IDC IDS IDSB IDST IDS_Binary_Operator IDS_Trinary_Operator ID_Continue ID_Start Ideo Ideographic Join_C Join_Control LOE Logical_Order_Exception Lower Lowercase Math NChar Noncharacter_Code_Point Pat_Syn Pat_WS Pattern_Syntax Pattern_White_Space QMark Quotation_Mark RI Radical Regional_Indicator SD STerm Sentence_Terminal Soft_Dotted Term Terminal_Punctuation UIdeo Unified_Ideograph Upper Uppercase VS Variation_Selector White_Space XIDC XIDS XID_Continue XID_Start space",
            "Extended_Pictographic",
            "",
            "EBase EComp EMod EPres ExtPict",
            "",
            "",
            "",
        )
    });

    static BIN_PROPERTY_OF_STRINGS_SETS: once_cell::unsync::Lazy<DataSet> = once_cell::unsync::Lazy::new(|| {
        DataSet::new(
            "",
            "",
            "",
            "",
            "",
            "",
            "Basic_Emoji Emoji_Keycap_Sequence RGI_Emoji RGI_Emoji_Flag_Sequence RGI_Emoji_Modifier_Sequence RGI_Emoji_Tag_Sequence RGI_Emoji_ZWJ_Sequence",
        )
    });
}

pub fn is_valid_unicode_property(
    version: EcmaVersion,
    name: &str,
    value: &str,
) -> bool {
    if GC_NAME_SET.contains(name) {
        return version >= EcmaVersion::_2018 &&
            GC_VALUE_SETS.with(|gc_value_sets| gc_value_sets.es2018().contains(value));
    }
    if SC_NAME_SET.contains(name) {
        return SC_VALUE_SETS.with(|sc_value_sets| {
            version >= EcmaVersion::_2018 && sc_value_sets.es2018().contains(value) ||
            version >= EcmaVersion::_2019 && sc_value_sets.es2019().contains(value) ||
            version >= EcmaVersion::_2020 && sc_value_sets.es2020().contains(value) ||
            version >= EcmaVersion::_2021 && sc_value_sets.es2021().contains(value) ||
            version >= EcmaVersion::_2022 && sc_value_sets.es2022().contains(value) ||
            version >= EcmaVersion::_2023 && sc_value_sets.es2023().contains(value)
        });
    }
    false
}

pub fn is_valid_lone_unicode_property(
    version: EcmaVersion,
    value: &str,
) -> bool {
    BIN_PROPERTY_SETS.with(|bin_property_sets| {
        version >= EcmaVersion::_2018 && bin_property_sets.es2018().contains(value) ||
        version >= EcmaVersion::_2019 && bin_property_sets.es2019().contains(value) ||
        version >= EcmaVersion::_2021 && bin_property_sets.es2021().contains(value)
    })
}

pub fn is_valid_lone_unicode_property_of_string(
    version: EcmaVersion,
    value: &str,
) -> bool {
    BIN_PROPERTY_OF_STRINGS_SETS.with(|bin_property_of_strings_sets| {
        version >= EcmaVersion::_2024 && bin_property_of_strings_sets.es2024().contains(value)
    })
}
