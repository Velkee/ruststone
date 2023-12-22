
use scraper::{Html, Selector};

fn make_request(url: &str) -> Result<String,reqwest::Error> {
    let response = reqwest::blocking::get(url)?;
    response.text()
}

#[derive(Debug)]
struct BasicCharacterInfo {
    name: String,
    world: String,
    lang: String,
    link: String
}

fn main() {
    let response = make_request("https://na.finalfantasyxiv.com/lodestone/character/?q=test&worldname=&classjob=&race_tribe=&blog_lang=ja&blog_lang=en&blog_lang=de&blog_lang=fr&order=").expect("Did not return valid response");
    
    let document = Html::parse_document(&response);

    let search_window_parser = Selector::parse("div.ldst__window").unwrap();
    let search_hit_parser = Selector::parse("div.entry").unwrap();
    let entry_parser = Selector::parse("a.entry__link").unwrap();
    let entry_box_parser = Selector::parse("div.entry__box.entry__box--world").unwrap();
    let entry_name_parser = Selector::parse("p.entry__name").unwrap();
    let entry_world_parser = Selector::parse("p.entry__world").unwrap();
    let entry_lang_parser = Selector::parse("div.entry__chara__lang").unwrap();

    let mut search_results: Vec<BasicCharacterInfo> = Vec::new();

    let search_result_table = document.select(&search_window_parser).next().expect("Didn't find the search result window.");
    for search_result in search_result_table.select(&search_hit_parser) {
        let entries = search_result.select(&entry_parser).collect::<Vec<_>>();
        for entry in entries {
            let mut character_info = BasicCharacterInfo {
                name: String::new(),
                world: String::new(),
                lang:  entry.select(&entry_lang_parser).next().unwrap().inner_html(),
                link: ("https://na.finalfantasyxiv.com".to_owned() + entry.value().attr("href").unwrap()),
            };

            let entry_box = entry.select(&entry_box_parser).next().unwrap();
            for entry_name in entry_box.select(&entry_name_parser) {
                character_info.name = entry_name.inner_html()
            }

            for entry_world in entry_box.select(&entry_world_parser) {
                character_info.world = entry_world.text().collect()
            }

            search_results.push(character_info)
        }
    }

    println!("{:#?}", search_results)
}
