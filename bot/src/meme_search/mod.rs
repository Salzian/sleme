use crate::{console_debug, console_log};
use get_template_error::GetTemplateError;
use meme_search_error::MemeSearchError;
use reqwest::get;
use scraper::{ElementRef, Html, Selector};

mod get_template_error;
mod meme_search_error;

pub(crate) async fn search_meme(query: &String, nth: usize) -> Result<String, MemeSearchError> {
    // Calculate the current page number from the requested ID. A page has 40 entries.
    let page = nth / 40 + 1;

    let url = format!("https://imgflip.com/memesearch?q={}&page={}", query, page);

    let response = get(url).await?;
    let text = response.text().await?;

    let document = Html::parse_document(&text);

    let memes_selector = Selector::parse("#mt-boxes-wrap > div.mt-boxes > div").unwrap();
    let select = document.select(&memes_selector);
    let elements = select.collect::<Vec<_>>();
    let selected_element = elements[nth - 1];
    let result = elements
        .iter()
        .map(|element| element.html())
        .collect::<Vec<_>>();
    console_log!("Found {} memes", result.len());

    let meme_url = get_meme_template_url(selected_element);
    let meme_id = get_meme_id_from_url(meme_url).await;
    console_debug!("Meme ID: {}", meme_id);

    // Get the ID of the meme from the memes' title href.
    //let id = get_meme_identifier(selected_element);

    // Get the full meme image from the meme template URL.
    //let meme_template = get_meme_template(id).await?;
    //console_debug!("Meme template: {}", meme_template);

    Ok("asdf".to_string())
}

async fn get_meme_id_from_url(url: String) -> i32 {
    let response = reqwest::get(url).await.unwrap();
    let text = response.text().await.unwrap();
    let document = Html::parse_document(&text);
    let selector = Selector::parse("#mtm-info > p:nth-child(4)").unwrap();
    let meme_id_element = document
        .select(&selector)
        .next()
        .expect("No meme ID found");
    let meme_id_text = meme_id_element
        .text()
        .collect::<Vec<_>>()
        .join("");
    let id = meme_id_text
        .trim_start_matches("Template ID: ")
        .parse::<i32>()
        .expect("Could not parse meme ID as integer");
    id
}

fn get_meme_template_url(element: ElementRef) -> String {
    let title_selector = Selector::parse("div.mt-img-wrap a").unwrap();
    let title = element.select(&title_selector).next().unwrap();
    console_debug!("Title: {}", title.html());

    let meme_slug = title
        .value()
        .attr("href")
        .expect("Meme title has no href attribute")
        .trim_start_matches("/meme/");
    console_debug!("href: {}", meme_slug);

    let url = format!("https://imgflip.com/memetemplate/{}", meme_slug);
    console_debug!("url: {}", url);
    url
}

fn get_meme_identifier(element: ElementRef) -> i32 {
    let title_selector = Selector::parse("div.mt-img-wrap a").unwrap();
    let title = element.select(&title_selector).next().unwrap();
    console_debug!("Title: {}", title.html());

    let href = title.value().attr("href").unwrap();
    console_debug!("href: {}", href);

    let id_regex = regex::Regex::new(r"^/meme/(\S+)").unwrap();
    let id_capture = id_regex
        .captures(href)
        .expect("Failed to extract ID from meme");
    console_debug!("id_capture: {:?}", id_capture);

    let id = id_capture
        .get(1)
        .expect("No ID found")
        .as_str()
        .parse::<i32>()
        .unwrap();
    id
}

async fn get_meme_template(id: i32) -> Result<String, GetTemplateError> {
    let url = format!("https://imgflip.com/memetemplate/{}", id);
    let response = get(url).await?;
    let text = response.text().await?;

    let document = Html::parse_document(&text);
    let image_selector = Selector::parse("img#im").unwrap();
    let video_selector = Selector::parse("#mtm-video > source").unwrap();

    let src = if let Some(image) = document.select(&image_selector).next() {
        image.value().attr("src").unwrap()
    } else if let Some(video) = document.select(&video_selector).next() {
        video.value().attr("src").unwrap()
    } else {
        return Err(GetTemplateError::NoMediaFound);
    };

    Ok(src.to_string())
}
