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
    let login_form=tab.wait_for_element("iframe > form > div#loginForm").unwrap();
    // login_form.wait_for_element("#login_id")?.type_str("WebKit")?;
    login_form.wait_for_element("#login_password")?;
    // let png_data=tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, Some(75),None,true)?;
    let png_data=login_form.capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
    fs::write("screenshot.png", png_data).unwrap();

    Ok(())
}

fn main() {
   match browse_page("https://www.accreditationnow.com/") {
    Err(e) =>println!("{}", e), 
    _ => ()
   }
}
