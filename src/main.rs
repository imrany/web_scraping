use headless_chrome::{
    Browser,
    protocol::cdp::Page,
};
use std::fs;
use anyhow::{Error, Result};

fn browse_page(url:&str)-> Result<(), Error>{
    let browser=Browser::default().unwrap();
    let tab=browser.new_tab().unwrap();

    // Navigate to url
    tab.navigate_to(url)?.wait_until_navigated()?;
    tab.wait_for_element("#userStatusPadding > a").expect("Failed to select element").click().expect("Failed to click");
    let iframe=tab.find_element("iframe")?;
    println!("{:?}",iframe);
    let png_data=tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, Some(75),None,true)?;
    fs::write("screenshot.png", png_data)?;

    Ok(())
}

fn main() {
   match browse_page("https://www.accreditationnow.com/") {
    Err(e) =>println!("{}", e), 
    _ => ()
   }
}
