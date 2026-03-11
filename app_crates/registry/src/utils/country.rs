// use fluent_templates::{LanguageIdentifier, Loader, static_loader};
use serde::{Deserialize, Serialize};

// static_loader! {
//     static TRANSLATIONS = {
//         locales: "./locales",
//         fallback_language: "en"
//     };
// }

macro_rules! define_countries {
    ($($variant:ident, $alpha2:expr, $alpha3:expr, $dial_code:expr);+ $(;)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub enum Country {
            $($variant),+
        }

        impl Country {
            pub const fn alpha2(&self) -> &'static str {
                match self {
                    $(Country::$variant => $alpha2),+
                }
            }

            pub const fn alpha3(&self) -> &'static str {
	            match self {
	                $(Country::$variant => $alpha3),+
	            }
            }

            /// Returns the dial code as a number (e.g., 1 for USA, 33 for France)
            pub const fn dial_code(&self) -> u16 {
                match self {
                    $(Country::$variant => $dial_code),+
                }
            }

            /// Returns the dial code formatted with + prefix (e.g., "+1", "+33")
            pub fn dial_code_formatted(&self) -> String {
                format!("+{}", self.dial_code())
            }

            pub fn from_alpha2(code: &str) -> Option<Self> {
                match code.to_uppercase().as_str() {
                    $($alpha2 => Some(Country::$variant)),+,
                    _ => None,
                }
            }

            pub fn from_alpha3(code: &str) -> Option<Self> {
            	match code.to_uppercase().as_str() {
                    $($alpha3 => Some(Country::$variant)),+,
                    _ => None,
                }
            }

            /// Returns a slice containing all countries.
            pub const fn all() -> &'static [Country] {
                &[$(Country::$variant),+]
            }
        }
    };
}

//Country list

define_countries![
    Afghanistan,                      "AF", "AFG",  93;
    Albania,                          "AL", "ALB",  355;
    Algeria,                          "DZ", "DZA",  213;
    Andorra,                          "AD", "AND",  376;
    Angola,                           "AO", "AGO",  244;
    AntiguaAndBarbuda,                "AG", "ATG",  1;
    Argentina,                        "AR", "ARG",  54;
    Armenia,                          "AM", "ARM",  374;
    Australia,                        "AU", "AUS",  61;
    Austria,                          "AT", "AUT",  43;
    Azerbaijan,                       "AZ", "AZE",  994;
    Bahamas,                          "BS", "BHS",  1;
    Bahrain,                          "BH", "BHR",  973;
    Bangladesh,                       "BD", "BGD",  880;
    Barbados,                         "BB", "BRB",  1;
    Belarus,                          "BY", "BLR",  375;
    Belgium,                          "BE", "BEL",  32;
    Belize,                           "BZ", "BLZ",  501;
    Benin,                            "BJ", "BEN",  229;
    Bhutan,                           "BT", "BTN",  975;
    Bolivia,                          "BO", "BOL",  591;
    BosniaAndHerzegovina,             "BA", "BIH",  387;
    Botswana,                         "BW", "BWA",  267;
    Brazil,                           "BR", "BRA",  55;
    Brunei,                           "BN", "BRN",  673;
    Bulgaria,                         "BG", "BGR",  359;
    BurkinaFaso,                      "BF", "BFA",  226;
    Burundi,                          "BI", "BDI",  257;
    Cambodia,                         "KH", "KHM",  855;
    Cameroon,                         "CM", "CMR",  237;
    Canada,                           "CA", "CAN",  1;
    CapeVerde,                        "CV", "CPV",  238;
    CentralAfricanRepublic,           "CF", "CAF",  236;
    Chad,                             "TD", "TCD",  235;
    Chile,                            "CL", "CHL",  56;
    China,                            "CN", "CHN",  86;
    Colombia,                         "CO", "COL",  57;
    Comoros,                          "KM", "COM",  269;
    CongoBrazzaville,                 "CG", "COG",  242;
    CongoKinshasa,                    "CD", "COD",  243;
    CostaRica,                        "CR", "CRI",  506;
    Croatia,                          "HR", "HRV",  385;
    Cuba,                             "CU", "CUB",  53;
    Cyprus,                           "CY", "CYP",  357;
    CzechRepublic,                    "CZ", "CZE",  420;
    Denmark,                          "DK", "DNK",  45;
    Djibouti,                         "DJ", "DJI",  253;
    Dominica,                         "DM", "DMA",  1;
    DominicanRepublic,                "DO", "DOM",  1;
    Ecuador,                          "EC", "ECU",  593;
    Egypt,                            "EG", "EGY",  20;
    ElSalvador,                       "SV", "SLV",  503;
    EquatorialGuinea,                 "GQ", "GNQ",  240;
    Eritrea,                          "ER", "ERI",  291;
    Estonia,                          "EE", "EST",  372;
    Eswatini,                         "SZ", "SWZ",  268;
    Ethiopia,                         "ET", "ETH",  251;
    Fiji,                             "FJ", "FJI",  679;
    Finland,                          "FI", "FIN",  358;
    France,                           "FR", "FRA",  33;
    Gabon,                            "GA", "GAB",  241;
    Gambia,                           "GM", "GMB",  220;
    Georgia,                          "GE", "GEO",  995;
    Germany,                          "DE", "DEU",  49;
    Ghana,                            "GH", "GHA",  233;
    Greece,                           "GR", "GRC",  30;
    Grenada,                          "GD", "GRD",  1;
    Guatemala,                        "GT", "GTM",  502;
    Guinea,                           "GN", "GIN",  224;
    GuineaBissau,                     "GW", "GNB",  245;
    Guyana,                           "GY", "GUY",  592;
    Haiti,                            "HT", "HTI",  509;
    Honduras,                         "HN", "HND",  504;
    Hungary,                          "HU", "HUN",  36;
    Iceland,                          "IS", "ISL",  354;
    India,                            "IN", "IND",  91;
    Indonesia,                        "ID", "IDN",  62;
    Iran,                             "IR", "IRN",  98;
    Iraq,                             "IQ", "IRQ",  964;
    Ireland,                          "IE", "IRL",  353;
    Israel,                           "IL", "ISR",  972;
    Italy,                            "IT", "ITA",  39;
    IvoryCoast,                       "CI", "CIV",  225;
    Jamaica,                          "JM", "JAM",  1;
    Japan,                            "JP", "JPN",  81;
    Jordan,                           "JO", "JOR",  962;
    Kazakhstan,                       "KZ", "KAZ",  7;
    Kenya,                            "KE", "KEN",  254;
    Kiribati,                         "KI", "KIR",  686;
    Kosovo,                           "XK", "XKS",  383;
    Kuwait,                           "KW", "KWT",  965;
    Kyrgyzstan,                       "KG", "KGZ",  996;
    Laos,                             "LA", "LAO",  856;
    Latvia,                           "LV", "LVA",  371;
    Lebanon,                          "LB", "LBN",  961;
    Lesotho,                          "LS", "LSO",  266;
    Liberia,                          "LR", "LBR",  231;
    Libya,                            "LY", "LBY",  218;
    Liechtenstein,                    "LI", "LIE",  423;
    Lithuania,                        "LT", "LTU",  370;
    Luxembourg,                       "LU", "LUX",  352;
    Madagascar,                       "MG", "MDG",  261;
    Malawi,                           "MW", "MWI",  265;
    Malaysia,                         "MY", "MYS",  60;
    Maldives,                         "MV", "MDV",  960;
    Mali,                             "ML", "MLI",  223;
    Malta,                            "MT", "MLT",  356;
    MarshallIslands,                  "MH", "MHL",  692;
    Mauritania,                       "MR", "MRT",  222;
    Mauritius,                        "MU", "MUS",  230;
    Mexico,                           "MX", "MEX",  52;
    Micronesia,                       "FM", "FSM",  691;
    Moldova,                          "MD", "MDA",  373;
    Monaco,                           "MC", "MCO",  377;
    Mongolia,                         "MN", "MNG",  976;
    Montenegro,                       "ME", "MNE",  382;
    Morocco,                          "MA", "MAR",  212;
    Mozambique,                       "MZ", "MOZ",  258;
    Myanmar,                          "MM", "MMR",  95;
    Namibia,                          "NA", "NAM",  264;
    Nauru,                            "NR", "NRU",  674;
    Nepal,                            "NP", "NPL",  977;
    Netherlands,                      "NL", "NLD",  31;
    NewZealand,                       "NZ", "NZL",  64;
    Nicaragua,                        "NI", "NIC",  505;
    Niger,                            "NE", "NER",  227;
    Nigeria,                          "NG", "NGA",  234;
    NorthKorea,                       "KP", "PRK",  850;
    NorthMacedonia,                   "MK", "MKD",  389;
    Norway,                           "NO", "NOR",  47;
    Oman,                             "OM", "OMN",  968;
    Pakistan,                         "PK", "PAK",  92;
    Palau,                            "PW", "PLW",  680;
    Palestine,                        "PS", "PSE",  970;
    Panama,                           "PA", "PAN",  507;
    PapuaNewGuinea,                   "PG", "PNG",  675;
    Paraguay,                         "PY", "PRY",  595;
    Peru,                             "PE", "PER",  51;
    Philippines,                      "PH", "PHL",  63;
    Poland,                           "PL", "POL",  48;
    Portugal,                         "PT", "PRT",  351;
    Qatar,                            "QA", "QAT",  974;
    Romania,                          "RO", "ROU",  40;
    Russia,                           "RU", "RUS",  7;
    Rwanda,                           "RW", "RWA",  250;
    SaintKittsAndNevis,               "KN", "KNA",  1;
    SaintLucia,                       "LC", "LCA",  1;
    SaintVincentAndTheGrenadines,     "VC", "VCT",  1;
    Samoa,                            "WS", "WSM",  685;
    SanMarino,                        "SM", "SMR",  378;
    SaoTomeAndPrincipe,               "ST", "STP",  239;
    SaudiArabia,                      "SA", "SAU",  966;
    Senegal,                          "SN", "SEN",  221;
    Serbia,                           "RS", "SRB",  381;
    Seychelles,                       "SC", "SYC",  248;
    SierraLeone,                      "SL", "SLE",  232;
    Singapore,                        "SG", "SGP",  65;
    Slovakia,                         "SK", "SVK",  421;
    Slovenia,                         "SI", "SVN",  386;
    SolomonIslands,                   "SB", "SLB",  677;
    Somalia,                          "SO", "SOM",  252;
    SouthAfrica,                      "ZA", "ZAF",  27;
    SouthKorea,                       "KR", "KOR",  82;
    SouthSudan,                       "SS", "SSD",  211;
    Spain,                            "ES", "ESP",  34;
    SriLanka,                         "LK", "LKA",  94;
    Sudan,                            "SD", "SDN",  249;
    Suriname,                         "SR", "SUR",  597;
    Sweden,                           "SE", "SWE",  46;
    Switzerland,                      "CH", "CHE",  41;
    Syria,                            "SY", "SYR",  963;
    Taiwan,                           "TW", "TWN",  886;
    Tajikistan,                       "TJ", "TJK",  992;
    Tanzania,                         "TZ", "TZA",  255;
    Thailand,                         "TH", "THA",  66;
    TimorLeste,                       "TL", "TLS",  670;
    Togo,                             "TG", "TGO",  228;
    Tonga,                            "TO", "TON",  676;
    TrinidadAndTobago,                "TT", "TTO",  1;
    Tunisia,                          "TN", "TUN",  216;
    Turkey,                           "TR", "TUR",  90;
    Turkmenistan,                     "TM", "TKM",  993;
    Tuvalu,                           "TV", "TUV",  688;
    Uganda,                           "UG", "UGA",  256;
    Ukraine,                          "UA", "UKR",  380;
    UnitedArabEmirates,               "AE", "ARE",  971;
    UnitedKingdom,                    "GB", "GBR",  44;
    UnitedStatesOfAmerica,            "US", "USA",  1;
    Uruguay,                          "UY", "URY",  598;
    Uzbekistan,                       "UZ", "UZB",  998;
    Vanuatu,                          "VU", "VUT",  678;
    VaticanCity,                      "VA", "VAT",  39;
    Venezuela,                        "VE", "VEN",  58;
    Vietnam,                          "VN", "VNM",  84;
    VirginIslandsBritish,             "VG", "VGB",  1;
    VirginIslandsUS,                  "VI", "VIR",  1;
    Yemen,                            "YE", "YEM",  967;
    Zambia,                           "ZM", "ZMB",  260;
    Zimbabwe,                         "ZW", "ZWE",  263;
];

impl Country {
    /// Returns the country name in English.
    /// Converts variant name from PascalCase to readable format.
    /// For example `Country::UnitedStatesOfAmerica.name()` returns `"United States of America"`.
    pub fn name(&self) -> String {
        let variant_name = format!("{self:?}");
        let mut result = String::new();
        for (i, c) in variant_name.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push(' ');
            }
            result.push(c);
        }
        result
    }

    /// Converts a Country into a flag emoji.
    /// For example Country::Bahamas.flag_emoji() == "🇧🇸"
    pub fn flag_emoji(&self) -> String {
        self.alpha2()
            .chars()
            .flat_map(|c| std::char::from_u32(0x1F1E6 + (c.to_ascii_uppercase() as u32 - 'A' as u32)))
            .collect()
    }

    // pub fn name(&self, language_identifier: &LanguageIdentifier) -> String {
    //     TRANSLATIONS.lookup(language_identifier, &format!("country-{}", self.alpha2()))
    // }
}

#[cfg(test)]
mod tests {
    // use fluent_templates::langid;

    use super::*;

    #[test]
    fn test_alpha2() {
        assert_eq!("DO", Country::DominicanRepublic.alpha2());
        assert_eq!("FR", Country::France.alpha2());
        assert_eq!("AE", Country::UnitedArabEmirates.alpha2());
    }

    #[test]
    fn test_alpha3() {
        assert_eq!("DOM", Country::DominicanRepublic.alpha3());
        assert_eq!("FRA", Country::France.alpha3());
        assert_eq!("ARE", Country::UnitedArabEmirates.alpha3());
    }

    #[test]
    fn test_from_alpha2() {
        assert_eq!(Some(Country::DominicanRepublic), Country::from_alpha2("DO"));
        assert_eq!(Some(Country::DominicanRepublic), Country::from_alpha2("Do"));
        assert_eq!(Some(Country::France), Country::from_alpha2("FR"));
        assert_eq!(Some(Country::France), Country::from_alpha2("fr"));
        assert_eq!(Some(Country::UnitedArabEmirates), Country::from_alpha2("AE"));
        assert_eq!(Some(Country::UnitedArabEmirates), Country::from_alpha2("aE"));

        assert_eq!(None, Country::from_alpha2("FZ"));
        assert_eq!(None, Country::from_alpha2("XX"));
    }

    #[test]
    fn test_from_alpha3() {
        assert_eq!(Some(Country::DominicanRepublic), Country::from_alpha3("DOM"));
        assert_eq!(Some(Country::DominicanRepublic), Country::from_alpha3("Dom"));
        assert_eq!(Some(Country::France), Country::from_alpha3("FRA"));
        assert_eq!(Some(Country::France), Country::from_alpha3("fra"));
        assert_eq!(Some(Country::UnitedArabEmirates), Country::from_alpha3("ARE"));
        assert_eq!(Some(Country::UnitedArabEmirates), Country::from_alpha3("arE"));

        assert_eq!(None, Country::from_alpha3("FZq"));
        assert_eq!(None, Country::from_alpha2("XXX"));
    }

    #[test]
    fn test_flag_emoji() {
        assert_eq!("🇩🇴", Country::DominicanRepublic.flag_emoji());
        assert_eq!("🇫🇷", Country::France.flag_emoji());
        assert_eq!("🇦🇪", Country::UnitedArabEmirates.flag_emoji());
    }

    // #[test]
    // fn test_name() {
    //     assert_eq!("فرنسا", &Country::France.name(&langid!("ar")));
    //     assert_eq!("Francia", &Country::France.name(&langid!("es")));
    //     assert_eq!("France", &Country::France.name(&langid!("en")));
    //     assert_eq!("France", &Country::France.name(&langid!("fr")));
    //     assert_eq!("फ्रांस", &Country::France.name(&langid!("hi")));
    //     assert_eq!("Pransya", &Country::France.name(&langid!("tl")));
    //     assert_eq!("فرانس", &Country::France.name(&langid!("ur")));
    //     assert_eq!("法国", &Country::France.name(&langid!("zh")));
    //     assert_eq!("France", &Country::France.name(&langid!("missing")));

    //     assert_eq!(
    //         "الإمارات العربية المتحدة",
    //         &Country::UnitedArabEmirates.name(&langid!("ar"))
    //     );
    //     assert_eq!(
    //         "United Arab Emirates",
    //         &Country::UnitedArabEmirates.name(&langid!("en"))
    //     );
    //     assert_eq!(
    //         "Émirats arabes unis",
    //         &Country::UnitedArabEmirates.name(&langid!("fr"))
    //     );
    // }
}

//
//
// #[cfg(all(test, feature = "ssr"))]
// mod sqlx_tests {
//     use sqlx::{Postgres, Type, TypeInfo, postgres::PgTypeInfo};

//     use super::*;

//     #[test]
//     fn test_type_compatibility() {
//         // Test that Country is compatible with text types
//         let country_binding = <Country as Type<sqlx::Postgres>>::type_info();
//         let country_type_name = country_binding.name();

//         let string_binding = <String as Type<sqlx::Postgres>>::type_info();
//         let text_type_name = string_binding.name();

//         assert_eq!(country_type_name, text_type_name);

//         // Test that the type system considers Country compatible with VARCHAR
//         let varchar_type = PgTypeInfo::with_name("VARCHAR");
//         assert!(<Country as Type<sqlx::Postgres>>::compatible(&varchar_type));
//     }

//     #[test]
//     fn test_type_info() {
//         let type_info = <Country as Type<Postgres>>::type_info();
//         let str_type_info = <&str as Type<Postgres>>::type_info();

//         assert_eq!(type_info.name(), str_type_info.name());
//     }

//     #[test]
//     fn test_compatible() {
//         let text_type_info = PgTypeInfo::with_name("text");

//         assert!(<Country as Type<Postgres>>::compatible(&text_type_info));
//     }

//     #[sqlx::test(migrations = "../../migrations")]
//     async fn test_decode(pool: sqlx::PgPool) -> Result<(), sqlx::Error> {
//         // Test decoding alpha2
//         let row: (Country,) = sqlx::query_as("SELECT 'FR'::text").fetch_one(&pool).await?;
//         assert_eq!(row.0, Country::France);

//         // Test decoding alpha3
//         let row: (Country,) = sqlx::query_as("SELECT 'FRA'::text").fetch_one(&pool).await?;
//         assert_eq!(row.0, Country::France);

//         // Test an invalid code
//         let maybe_row =
//             sqlx::query_as::<_, (Country,)>("SELECT 'XYZ'::text").fetch_one(&pool).await;
//         assert!(maybe_row.is_err());

//         Ok(())
//     }
// }
