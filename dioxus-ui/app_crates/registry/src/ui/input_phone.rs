use dioxus::prelude::*;
use icons::{ChevronsUpDown, Search, X};
use serde::{Deserialize, Serialize};
use tw_merge::tw_merge;

use crate::ui::command::{
    Command, CommandEmpty, CommandGroup, CommandGroupLabel, CommandInput, CommandItem, CommandList,
};
use crate::ui::input::Input;
use crate::ui::popover::{Popover, PopoverContent, PopoverTrigger};

/* ========================================================== */
/*                  ✨ INLINE UTILS ✨                        */
/* ========================================================== */

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

            pub const fn dial_code(&self) -> u16 {
                match self {
                    $(Country::$variant => $dial_code),+
                }
            }

            pub fn dial_code_formatted(&self) -> String {
                format!("+{}", self.dial_code())
            }

            pub const fn all() -> &'static [Country] {
                &[$(Country::$variant),+]
            }
        }
    };
}

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

    pub fn flag_emoji(&self) -> String {
        self.alpha2()
            .chars()
            .flat_map(|c| std::char::from_u32(0x1F1E6 + (c.to_ascii_uppercase() as u32 - 'A' as u32)))
            .collect()
    }

    pub const fn trunk_prefix(&self) -> Option<&'static str> {
        match self {
            Country::UnitedStatesOfAmerica
            | Country::Canada
            | Country::Bahamas
            | Country::Barbados
            | Country::DominicanRepublic
            | Country::Jamaica
            | Country::TrinidadAndTobago => None,
            Country::Italy | Country::SanMarino | Country::VaticanCity => None,
            Country::Denmark
            | Country::Norway
            | Country::Iceland
            | Country::Liechtenstein
            | Country::Monaco
            | Country::Andorra => None,
            _ => Some("0"),
        }
    }
}

/// A phone number containing only digits.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    pub fn new(input: &str, max_digits: usize) -> Self {
        Self(input.chars().filter(|c| c.is_ascii_digit()).take(max_digits).collect())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn format(&self, country: Country) -> String {
        PhoneFormat::for_country(country).format(&self.0)
    }

    pub fn format_international(&self, country: Country) -> String {
        if self.0.is_empty() {
            return String::new();
        }
        let subscriber = match country.trunk_prefix() {
            Some(prefix) => self.0.strip_prefix(prefix).unwrap_or(&self.0),
            None => &self.0,
        };
        format!("{} {}", country.dial_code_formatted(), PhoneFormat::for_country(country).format(subscriber))
    }
}

pub struct PhoneFormat {
    pub groups: &'static [usize],
    pub max_digits: usize,
}

impl PhoneFormat {
    pub const fn for_country(country: Country) -> Self {
        match country {
            Country::UnitedStatesOfAmerica
            | Country::Canada
            | Country::Bahamas
            | Country::Barbados
            | Country::DominicanRepublic
            | Country::Jamaica
            | Country::TrinidadAndTobago => Self { groups: &[3, 3, 4], max_digits: 10 },
            Country::France => Self { groups: &[1, 2, 2, 2, 2], max_digits: 9 },
            Country::UnitedKingdom => Self { groups: &[4, 3, 3], max_digits: 10 },
            Country::Germany => Self { groups: &[3, 3, 4], max_digits: 10 },
            Country::Thailand => Self { groups: &[2, 3, 4], max_digits: 9 },
            Country::Japan => Self { groups: &[2, 4, 4], max_digits: 10 },
            Country::China => Self { groups: &[3, 4, 4], max_digits: 11 },
            Country::India => Self { groups: &[5, 5], max_digits: 10 },
            Country::Australia => Self { groups: &[3, 3, 3], max_digits: 9 },
            Country::Brazil => Self { groups: &[2, 5, 4], max_digits: 11 },
            Country::Mexico => Self { groups: &[2, 4, 4], max_digits: 10 },
            Country::Spain => Self { groups: &[3, 3, 3], max_digits: 9 },
            Country::Italy => Self { groups: &[3, 3, 4], max_digits: 10 },
            _ => Self { groups: &[3, 3, 4], max_digits: 15 },
        }
    }

    pub fn format(&self, digits: &str) -> String {
        let mut result = String::new();
        let mut chars = digits.chars().peekable();

        for (i, &group_size) in self.groups.iter().enumerate() {
            if chars.peek().is_none() {
                break;
            }
            if i > 0 {
                result.push(' ');
            }
            for _ in 0..group_size {
                if let Some(c) = chars.next() {
                    result.push(c);
                } else {
                    break;
                }
            }
        }

        for c in chars {
            result.push(c);
        }

        result
    }

    pub fn placeholder(&self) -> String {
        let digits: String = (0..self.max_digits).map(|i| char::from(b'0' + (i % 10) as u8)).collect();
        self.format(&digits)
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const COMMON_COUNTRIES: &[Country] = &[
    Country::UnitedStatesOfAmerica,
    Country::UnitedKingdom,
    Country::France,
    Country::Germany,
    Country::Canada,
    Country::Australia,
    Country::Spain,
    Country::Italy,
    Country::Japan,
    Country::China,
    Country::India,
    Country::Brazil,
    Country::Mexico,
];

/* ========================================================== */
/*                     ✨ CLX COMPONENTS ✨                   */
/* ========================================================== */

#[component]
pub fn InputPhoneWrapper(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged = tw_merge!("flex w-full", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "InputPhoneWrapper", class: "{merged}", {children} } }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn CountryItem(country: Country, selected_country: Signal<Country>) -> Element {
    let search_value = format!("{} {} {}", country.name(), country.alpha2(), country.dial_code_formatted());
    let is_selected = selected_country() == country;

    rsx! {
        CommandItem {
            value: search_value,
            selected: is_selected,
            reserve_check_space: true,
            on_select: move |_| {
                selected_country.set(country);
            },
            span { class: "text-base", "{country.flag_emoji()}" }
            span { class: "flex-1 truncate", "{country.name()}" }
            span { class: "w-12 text-right text-muted-foreground", "{country.dial_code_formatted()}" }
        }
    }
}

#[component]
pub fn InputPhone(
    #[props(into, optional)] class: Option<String>,
    #[props(optional)] value_signal: Option<Signal<PhoneNumber>>,
    #[props(optional)] country_signal: Option<Signal<Country>>,
    #[props(optional)] disabled: bool,
    #[props(optional)] invalid: bool,
    #[props(optional)] on_blur: Option<EventHandler<()>>,
) -> Element {
    let internal_value_signal = use_signal(PhoneNumber::default);
    let internal_country_signal = use_signal(|| Country::UnitedStatesOfAmerica);

    let mut value = value_signal.unwrap_or(internal_value_signal);
    let selected_country = country_signal.unwrap_or(internal_country_signal);

    let wrapper_class = tw_merge!("flex w-full", class.as_deref().unwrap_or(""));
    let country = selected_country();
    let placeholder = PhoneFormat::for_country(country).placeholder();
    let formatted_value = value().format(country);
    let flag = country.flag_emoji();
    let dial = country.dial_code_formatted();
    let is_empty = value().is_empty();

    rsx! {
        InputPhoneWrapper { class: wrapper_class,
            Popover {
                PopoverTrigger {
                    class: "gap-1 px-3 w-auto rounded-r-none border-r-0",
                    disabled: disabled,
                    aria_label: "Select country",
                    span { class: "text-base", "{flag}" }
                    span { class: "text-xs text-muted-foreground", "{dial}" }
                    ChevronsUpDown { class: "ml-1 opacity-50 size-3" }
                }

                PopoverContent { class: "p-0 w-[280px]",
                    Command {
                        div { class: "flex gap-2 items-center px-2 border-b",
                            Search { class: "size-4 text-muted-foreground shrink-0" }
                            CommandInput { }
                        }
                        CommandList { class: "min-h-0 max-h-[280px]",
                            CommandEmpty { "No country found." }

                            // Common countries
                            CommandGroup {
                                for &country in COMMON_COUNTRIES {
                                    CountryItem { country, selected_country }
                                }
                            }

                            // Separator + rest of countries
                            CommandGroup {
                                CommandGroupLabel { "All countries" }
                                for &country in Country::all().iter().filter(|c| !COMMON_COUNTRIES.contains(c)) {
                                    CountryItem { country, selected_country }
                                }
                            }
                        }
                    }
                }
            }

            // Phone number input - displays formatted, stores raw digits
            div { class: "relative flex-1",
                Input {
                    class: "pr-8 w-full rounded-l-none",
                    r#type: crate::ui::input::InputType::Tel,
                    placeholder: placeholder,
                    disabled: disabled,
                    value: formatted_value,
                    oninput: move |ev: FormEvent| {
                        let format = PhoneFormat::for_country(selected_country());
                        let phone = PhoneNumber::new(&ev.value(), format.max_digits);
                        value.set(phone);
                    },
                }
                if is_empty || disabled {
                    {}
                } else {
                    button {
                        r#type: "button",
                        tabindex: "-1",
                        class: "absolute right-2 top-1/2 p-0.5 rounded-sm transition-colors -translate-y-1/2 text-muted-foreground hover:text-foreground hover:bg-muted",
                        "aria-label": "Clear phone number",
                        onclick: move |_| value.set(PhoneNumber::default()),
                        X { class: "size-4" }
                    }
                }
            }
        }
    }
}
