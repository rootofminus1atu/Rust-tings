use scraper::{Element, ElementRef, Html, Selector};
use chrono::{NaiveTime, TimeDelta};

fn main() {
    println!("hi the html is");

    let lessons = get_lessons(HTML_STRING).expect("buuuhhh");

    dbg!(&lessons);
}

/// Retrieves a collection of Lessons, given an html string
fn get_lessons(html_str: &str) -> Result<Vec<Vec<Lesson>>, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html_str);

    let week_day_elements = get_week_day_elements(&document)?;

    let timetable_cells = get_timetable_cells(week_day_elements)?;

    let lessons = process_timetable_cells(timetable_cells)?;

    Ok(lessons)
}

/// Targets the Mon, Tue, Wed, etc. elements, so that we could have the lesson rows ready to process
fn get_week_day_elements<'a>(document: &'a Html) -> Result<Vec<ElementRef<'a>>, Box<dyn std::error::Error>> {
    let week_hour_selector = Selector::parse("font[color='#FFFFFF']").unwrap();
    let mut candidates = document.select(&week_hour_selector);

    let week_day_texts = vec!["Mon", "Tue", "Wed", "Thu", "Fri"];
    let mut week_day_elems = vec![];

    for text in week_day_texts {
        match candidates.find(|elem| elem.text().collect::<String>().contains(text)) {
            Some(elem) => {
                week_day_elems.push(elem.parent_element().unwrap())

            },
            None => return Err(format!("Error while getting week day elements. Did not find a `{}` cell", text).into())
        }
    }

    Ok(week_day_elems)
}

/// Retrieves all the cells that could contain any lessons, given a collection of weekday elements, provided by `get_week_day_elements` 
fn get_timetable_cells<'a>(week_day_elements: Vec<ElementRef<'a>>) -> Result<Vec<Vec<ElementRef<'a>>>, Box<dyn std::error::Error>> {
    let mut week_container = vec![];

    for mut week_day in week_day_elements {
        // skipping the 1st row, which contains Mon, Tue, Wed, etc.
        week_day = week_day.next_sibling_element().unwrap();

        let mut day_container = vec![];

        // just traversing the siblings, probably a suboptimal way but oh well
        let mut current_sibling = Some(week_day);
        while let Some(sibling) = current_sibling {
            day_container.push(sibling);
            current_sibling = sibling.next_sibling_element();
        }

        week_container.push(day_container);
    }

    Ok(week_container)
}

/// Parses a colspan
/// 
/// A colspan is responsible for decoding how long a particular lesson would last for, with the rule
/// `1 colspan = 30 minutes`
fn parse_colspan(elem: &ElementRef<'_>) -> Result<Option<i32>, Box<dyn std::error::Error>> {
    match elem.attr("colspan") {
        Some(s) => {
            // we want to return an Err only if the parsing fails, somehow
            let parsed = s.parse::<i32>()?;  
            Ok(Some(parsed))
        },
        None => Ok(None)
    }
}

#[derive(Debug)]
struct Lesson {
    start: NaiveTime,
    duration: i32,
    day: i32,
    details: LessonDetails
}

impl Lesson {
    pub fn new(start: NaiveTime, duration: i32, day: i32, details: LessonDetails) -> Self {
        Self { start, duration, day, details }
    }
}

#[derive(Debug)]
struct LessonDetails {
    subject: String,
    room_id: String,
    room_desc: String,
    room_cap: i32,
    lecturer: String,
    idk_what_this_means: String
}

impl LessonDetails {
    pub fn new(subject: String, room_id: String, room_desc: String, room_cap: i32, lecturer: String, idk_what_this_means: String) -> Self {
        Self { subject, room_id, room_desc, room_cap, lecturer, idk_what_this_means }
    }

    pub fn from_preprocessed(data: LessonPreProcessedDetails) -> Result<LessonDetails, Box<dyn std::error::Error>> {
        let subject = data.subject;
        let (room_id, room_desc, room_cap) = Self::parse_room_details(&data.room_details)?;
        let lecturer = data.lecturer;
        let idk_what_this_means = data.last_line;

        Ok(Self::new(subject, room_id, room_desc, room_cap, lecturer, idk_what_this_means))
    }

    pub fn parse_room_details(room_details: &str) -> Result<(String, String, i32), Box<dyn std::error::Error>> {
        let parts = room_details.split_once("-")
            .ok_or(format!("Error while parsing room details. Invalid format: `{}`", room_details))?;
    
        let room_id = parts.0.trim().to_string();
    
        let second_part: Vec<&str> = parts.1.split(['(', ')'].as_ref()).map(|s| s.trim()).collect();
    
        if second_part.len() != 3 {
            return Err("Error while parsing room details. Invalid format".into());
        }
    
        let room_desc = second_part[0].trim().to_string();
        let room_cap = second_part[1]
            .parse::<i32>()
            .map_err(|_| "Error while parsing room details. Failed to parse number of seats")?;
        // second_part[2] is just an empty string

        Ok((room_id, room_desc, room_cap))
    }
}

#[derive(Debug)]
struct LessonPreProcessedDetails {
    subject: String,
    room_details: String,
    lecturer: String,
    last_line: String
}

impl LessonPreProcessedDetails {
    pub fn new(subject: String, room_details: String, lecturer: String, last_line: String) -> Self {
        Self { subject, room_details, lecturer, last_line }
    }

    /// Assumptions:
    /// - The details of a lesson are kept in <font> elements
    /// - There's always 4 such elements
    pub fn from_element(element: &ElementRef<'_>) -> Result<Self, Box<dyn std::error::Error>> {
        let font_selector = Selector::parse("font").unwrap();
        let found = element.select(&font_selector)
            .map(|e| e.inner_html())
            .collect::<Vec<_>>();

        if found.len() != 4 {
            return Err(format!("Error while extracting pre-processed lesson details. Expected 4 strings but found {} instead.", found.len()).into());
        }

        let [subject, room_details, lecturer, last_line] = found.try_into().unwrap();

        let preprocessed_details = Self::new(subject, room_details, lecturer, last_line);

        Ok(preprocessed_details)
    }
}

/// Processes a given collection of timetable cells and produces lessons based on those
/// 
/// Assumptions:
/// - A lesson can NOT last longer than 1 hour
fn process_timetable_cells(timetable_cells: Vec<Vec<ElementRef<'_>>>) -> Result<Vec<Vec<Lesson>>, Box<dyn std::error::Error>> {
    let lessons = timetable_cells.iter()
        .enumerate()
        .map(|(i, day)| {
            // we use this starting time in conjunction with the colspan variable to determine when lessons start and how long they are
            let mut time = NaiveTime::from_hms_opt(9, 0, 0).unwrap();

            let mut lessons_in_day = vec![];

            for cell in day {
                // to see what colspan means, check its doc
                let colspan = parse_colspan(cell)?;

                // read the below as "if there is a lesson going on"
                if let Some(colspan_num) = colspan { 
                    let duration = colspan_num / 2;

                    // extract more data
                    // maybe the following 2 lines could be moves inside the Lesson struct
                    let data = LessonPreProcessedDetails::from_element(cell)?;
                    let processed = LessonDetails::from_preprocessed(data)?;

                    let lesson = Lesson::new(time, duration, i as i32 + 1, processed);

                    lessons_in_day.push(lesson);
                }
                
                match colspan {
                    Some(n) => time += TimeDelta::minutes(n as i64 * 30),
                    None => time += TimeDelta::minutes(30)
                }
            }

            Ok(lessons_in_day)
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;

    Ok(lessons)
}


const HTML_STRING: &str = r###"j"###;