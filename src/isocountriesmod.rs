//! **isocountriesmod - country names and iso code**  

//region use
use crate::log1;
use serde::{Deserialize, Serialize};
//endregion

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    name: String,
    code: String,
}

/// get the name from this json
pub fn get_country_name(iso_code: &str) -> String {
    //this is not too big. It adds only 10kb to the wasm, that is already 498kb
    let json = r#"[
{"name": "Afghanistan ", "code": "AF"},
{"name": "Aland Islands", "code": "AX"},
{"name": "Albania", "code": "AL"},
{"name": "Algeria", "code": "DZ"},
{"name": "Samoa Americane", "code": "AS"},
{"name": "Andorra", "code": "AD"},
{"name": "Angola", "code": "AO"},
{"name": "Anguilla", "code": "AI"},
{"name": "Antartide", "code": "AQ"},
{"name": "Antigua e Barbuda", "code": "AG"},
{"name": "Argentina", "code": "AR"},
{"name": "Armenia", "code": "AM"},
{"name": "Aruba", "code": "AW"},
{"name": "Australia", "code": "AU"},
{"name": "Austria", "code": "AT"},
{"name": "Azerbaijan", "code": "AZ"},
{"name": "Bahamas", "code": "BS"},
{"name": "Bahrein", "code": "BH"},
{"name": "Bangladesh", "code": "BD"},
{"name": "Barbados", "code": "BB"},
{"name": "Bielorussia", "code": "BY"},
{"name": "Belgio", "code": "BE"},
{"name": "Belize", "code": "BZ"},
{"name": "Benin", "code": "BJ"},
{"name": "Bermuda", "code": "BM"},
{"name": "Bhutan", "code": "BT"},
{"name": "Bolivia", "code": "BO"},
{"name": "Bosnia ed Erzegovina", "code": "BA"},
{"name": "Botswana", "code": "BW"},
{"name": "Isola Bouvet", "code": "BV"},
{"name": "Brasile", "code": "BR"},
{"name": "Territori Britannici dell'Oceano Indiano", "code": "IO"},
{"name": "Brunei", "code": "BN"},
{"name": "Bulgaria", "code": "BG"},
{"name": "Burkina Faso", "code": "BF"},
{"name": "Burundi", "code": "BI"},
{"name": "Cambogia", "code": "KH"},
{"name": "Camerun", "code": "CM"},
{"name": "Canada", "code": "CA"},
{"name": "Capo Verde", "code": "CV"},
{"name": "Isole Cayman", "code": "KY"},
{"name": "Repubblica Centrafricana", "code": "CF"},
{"name": "Ciad", "code": "TD"},
{"name": "Cile", "code": "CL"},
{"name": "Cina", "code": "CN"},
{"name": "Isola di Natale", "code": "CX"},
{"name": "Isole Cocos", "code": "CC"},
{"name": "Colombia", "code": "CO"},
{"name": "Comore", "code": "KM"},
{"name": "Repubblica del Congo", "code": "CG"},
{"name": "Repubblica Democratica del Congo", "code": "CD"},
{"name": "Isole Cook", "code": "CK"},
{"name": "Costa Rica", "code": "CR"},
{"name": "osta d'Avorio", "code": "CI"},
{"name": "Croazia", "code": "HR"},
{"name": "Cuba", "code": "CU"},
{"name": "Cipro", "code": "CY"},
{"name": "Repubblica Ceca", "code": "CZ"},
{"name": "Danimarca", "code": "DK"},
{"name": "Gibuti", "code": "DJ"},
{"name": "Dominica", "code": "DM"},
{"name": "Repubblica Dominicana", "code": "DO"},
{"name": "Ecuador", "code": "EC"},
{"name": "Egitto", "code": "EG"},
{"name": "El Salvador", "code": "SV"},
{"name": "Guinea Equatoriale", "code": "GQ"},
{"name": "Eritrea", "code": "ER"},
{"name": "Estonia", "code": "EE"},
{"name": "Etiopia", "code": "ET"},
{"name": "Isole Falkland", "code": "FK"},
{"name": "Isole Faroe", "code": "FO"},
{"name": "Isole Fiji", "code": "FJ"},
{"name": "Finlandia", "code": "FI"},
{"name": "Francia", "code": "FR"},
{"name": "Guyana Francese", "code": "GF"},
{"name": "Polinesia Francese", "code": "PF"},
{"name": "Terre Australi e Antartiche Francesi", "code": "TF"},
{"name": "Gabon", "code": "GA"},
{"name": "Gambia", "code": "GM"},
{"name": "Georgia", "code": "GE"},
{"name": "Germania", "code": "DE"},
{"name": "Ghana", "code": "GH"},
{"name": "Gibilterra", "code": "GI"},
{"name": "Grecia", "code": "GR"},
{"name": "Groenlandia", "code": "GL"},
{"name": "Grenada", "code": "GD"},
{"name": "Guadalupa", "code": "GP"},
{"name": "Guam", "code": "GU"},
{"name": "Guatemala", "code": "GT"},
{"name": "Guernsey", "code": "GG"},
{"name": "Guinea", "code": "GN"},
{"name": "Guinea Bissau", "code": "GW"},
{"name": "Guyana", "code": "GY"},
{"name": "Haiti", "code": "HT"},
{"name": "Isole Heard e McDonald", "code": "HM"},
{"name": "Holy See (Vatican City State)", "code": "VA"},
{"name": "Honduras", "code": "HN"},
{"name": "Hong Kong", "code": "HK"},
{"name": "Ungheria", "code": "HU"},
{"name": "Islanda", "code": "IS"},
{"name": "India", "code": "IN"},
{"name": "Indonesia", "code": "ID"},
{"name": "Iran", "code": "IR"},
{"name": "Iraq", "code": "IQ"},
{"name": "Irlanda", "code": "IE"},
{"name": "Isola di Man", "code": "IM"},
{"name": "Israele", "code": "IL"},
{"name": "Italia", "code": "IT"},
{"name": "Giamaica", "code": "JM"},
{"name": "Giappone", "code": "JP"},
{"name": "Jersey", "code": "JE"},
{"name": "Giordania", "code": "JO"},
{"name": "Kazakhstan", "code": "KZ"},
{"name": "Kenya", "code": "KE"},
{"name": "Kiribati", "code": "KI"},
{"name": "Repubblica Popolare Democratica di Corea (Corea del Nord)", "code": "KP"},
{"name": "Repubblica di Corea (Corea del Sud)", "code": "KR"},
{"name": "Kuwait", "code": "KW"},
{"name": "Kirghizistan", "code": "KG"},
{"name": "Laos (Repubblica Popolare Democratica del Laos)", "code": "LA"},
{"name": "Lettonia", "code": "LV"},
{"name": "Libano", "code": "LB"},
{"name": "Lesotho", "code": "LS"},
{"name": "Liberia", "code": "LR"},
{"name": "Libia", "code": "LY"},
{"name": "Liechtenstein", "code": "LI"},
{"name": "Lituania", "code": "LT"},
{"name": "Lussemburgo", "code": "LU"},
{"name": "Macao", "code": "MO"},
{"name": "Repubblica di Macedonia", "code": "MK"},
{"name": "Madagascar", "code": "MG"},
{"name": "Malawi", "code": "MW"},
{"name": "Malesia", "code": "MY"},
{"name": "Maldive", "code": "MV"},
{"name": "Mali", "code": "ML"},
{"name": "Malta", "code": "MT"},
{"name": "Isole Marshall", "code": "MH"},
{"name": "Martinica", "code": "MQ"},
{"name": "Mauritania", "code": "MR"},
{"name": "Mauritius", "code": "MU"},
{"name": "Mayotte", "code": "YT"},
{"name": "Messico", "code": "MX"},
{"name": "Micronesia (Isole)", "code": "FM"},
{"name": "Moldova", "code": "MD"},
{"name": "Monaco", "code": "MC"},
{"name": "Mongolia", "code": "MN"},
{"name": "Montserrat", "code": "MS"},
{"name": "Marocco", "code": "MA"},
{"name": "Mozambico", "code": "MZ"},
{"name": "Birmania (Myanmar)", "code": "MM"},
{"name": "Namibia", "code": "NA"},
{"name": "Nauru", "code": "NR"},
{"name": "Nepal", "code": "NP"},
{"name": "Paesi Bassi (Olanda)", "code": "NL"},
{"name": "Antille Olandesi", "code": "AN"},
{"name": "Nuova Caledonia", "code": "NC"},
{"name": "Nuova Zelanda", "code": "NZ"},
{"name": "Nicaragua", "code": "NI"},
{"name": "Niger", "code": "NE"},
{"name": "Nigeria", "code": "NG"},
{"name": "Niue", "code": "NU"},
{"name": "Isola Norfolk", "code": "NF"},
{"name": "Isole Marianne Settentrionali", "code": "MP"},
{"name": "Norvegia", "code": "NO"},
{"name": "Oman", "code": "OM"},
{"name": "Pakistan", "code": "PK"},
{"name": "Palau", "code": "PW"},
{"name": "Territori palestinesi", "code": "PS"},
{"name": "Panama", "code": "PA"},
{"name": "Papua Nuova Guinea", "code": "PG"},
{"name": "Paraguay", "code": "PY"},
{"name": "Peru", "code": "PE"},
{"name": "Filippine", "code": "PH"},
{"name": "Isole Pitcairn", "code": "PN"},
{"name": "Polonia", "code": "PL"},
{"name": "Portogallo", "code": "PT"},
{"name": "Porto Rico", "code": "PR"},
{"name": "Qatar", "code": "QA"},
{"name": "Riunione (isola)", "code": "RE"},
{"name": "Romania", "code": "RO"},
{"name": "Russia", "code": "RU"},
{"name": "Ruanda", "code": "RW"},
{"name": "Saint Helena", "code": "SH"},
{"name": "Saint Kitts and Nevis", "code": "KN"},
{"name": "Santa Lucia", "code": "LC"},
{"name": "Saint Pierre and Miquelon", "code": "PM"},
{"name": "Saint Vincent and the Grenadines", "code": "VC"},
{"name": "Samoa (Samoa Occidentali)", "code": "WS"},
{"name": "San Marino", "code": "SM"},
{"name": "Sao Tome and Principe", "code": "ST"},
{"name": "Arabia Saudita", "code": "SA"},
{"name": "Senegal", "code": "SN"},
{"name": "Serbia", "code": "CS"},
{"name": "Seychelles", "code": "SC"},
{"name": "Sierra Leone", "code": "SL"},
{"name": "Singapore", "code": "SG"},
{"name": "Slovacchia", "code": "SK"},
{"name": "Slovenia", "code": "SI"},
{"name": "Isole Salomone", "code": "SB"},
{"name": "Somalia", "code": "SO"},
{"name": "Sudafrica", "code": "ZA"},
{"name": "Georgia del Sud e Isole Sandwich Australi", "code": "GS"},
{"name": "Spagna", "code": "ES"},
{"name": "Sri Lanka", "code": "LK"},
{"name": "Sudan", "code": "SD"},
{"name": "Suriname", "code": "SR"},
{"name": "Svalbard and Jan Mayen", "code": "SJ"},
{"name": "Swaziland", "code": "SZ"},
{"name": "Svezia", "code": "SE"},
{"name": "Svizzera", "code": "CH"},
{"name": "Siria, Repubblica Araba di Siria", "code": "SY"},
{"name": "Taiwan, Repubblica di Cina", "code": "TW"},
{"name": "Tagikistan", "code": "TJ"},
{"name": "Tanzania", "code": "TZ"},
{"name": "Thailandia", "code": "TH"},
{"name": "Timor Est", "code": "TL"},
{"name": "Togo", "code": "TG"},
{"name": "Tokelau", "code": "TK"},
{"name": "Tonga", "code": "TO"},
{"name": "Trinidad e Tobago", "code": "TT"},
{"name": "Tunisia", "code": "TN"},
{"name": "Turchia", "code": "TR"},
{"name": "Turkmenistan", "code": "TM"},
{"name": "Isole Turks and Caicos", "code": "TC"},
{"name": "Tuvalu", "code": "TV"},
{"name": "Uganda", "code": "UG"},
{"name": "Ucraina", "code": "UA"},
{"name": "Emirati Arabi Uniti", "code": "AE"},
{"name": "Regno Unito", "code": "GB"},
{"name": "Stati Uniti d'America", "code": "US"},
{"name": "Isole minori esterne degli Stati Uniti d'America", "code": "UM"},
{"name": "Uruguay", "code": "UY"},
{"name": "Uzbekistan", "code": "UZ"},
{"name": "Vanuatu", "code": "VU"},
{"name": "Venezuela", "code": "VE"},
{"name": "Vietnam", "code": "VN"},
{"name": "Isole Vergini Britanniche", "code": "VG"},
{"name": "Isole Vergini Statunitensi", "code": "VI"},
{"name": "Wallis e Futuna", "code": "WF"},
{"name": "Sahara Occidentale", "code": "EH"},
{"name": "Yemen", "code": "YE"},
{"name": "Zambia", "code": "ZM"},
{"name": "Zimbabwe", "code": "ZW"}
]"#;
    //TODO: create the vector only once and have it in memory. But where and how?
    let p: Vec<Country> = unwrap!(serde_json::from_str(json));
    //search by iso_code and return name
    let x = unwrap!(p.iter().find(|r| r.code == iso_code));
    log1(&format!("{:?}", x));
    //return
    x.name.clone()
}