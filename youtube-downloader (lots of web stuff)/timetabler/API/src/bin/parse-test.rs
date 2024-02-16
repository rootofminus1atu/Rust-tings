use scraper::{Element, ElementRef, Html, Selector};
use chrono::NaiveTime;

fn main() {
    println!("hi the html is");

    let document = Html::parse_fragment(html_str);

    let cells = get_timetable_cells(&document).expect("buuuhhh");
    
}

fn get_week_day_elems<'a>(document: &'a Html) -> Result<Vec<ElementRef<'a>>, Box<dyn std::error::Error>> {
    let week_hour_selector = Selector::parse("font[color='#FFFFFF']").unwrap();
    let mut candidates = document.select(&week_hour_selector);

    let week_day_texts = vec!["Mon", "Tue", "Wed", "Thu", "Fri"];
    let mut week_day_elems = vec![];

    for text in week_day_texts {
        match candidates.find(|elem| elem.text().collect::<String>().contains(text)) {
            Some(elem) => {
                week_day_elems.push(elem.parent_element().unwrap())

            },
            None => return Err(format!("Did not find a {} cell", text).into())
        }
    }

    Ok(week_day_elems)
}

fn get_timetable_cells<'a>(document: &'a Html) -> Result<Vec<Vec<ElementRef<'a>>>, Box<dyn std::error::Error>> {
    let week_day_elems = get_week_day_elems(&document)?;

    let mut week_container = vec![];

    for mut week_day in week_day_elems {
        // skipping the 1st cell (the week day name cell)
        week_day = week_day.next_sibling_element().unwrap();

        let mut day_container = vec![];

        while week_day.next_sibling_element().is_some() {
            day_container.push(week_day);
            week_day = week_day.next_sibling_element().unwrap();
        }

        week_container.push(day_container);
    }

    Ok(week_container)
}

fn process_timetable_cells<'a>(timetable_cells: Vec<Vec<ElementRef<'a>>>) -> Result<(), Box<dyn std::error::Error>> {

    Ok(())
}


