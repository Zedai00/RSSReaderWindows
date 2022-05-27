use windows::{
    Foundation::Uri, Web::Syndication::SyndicationClient, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;

    let mut text = String::new();
    for item in feed.Items()? {
        // println!("{}", item.Title()?.Text()?);
        let t = format!("{}\n", item.Title()?.Text()?.to_string());
        let str = &t[..];
        text.push_str(str);
    }

    unsafe {
        MessageBoxA(None, text, "Rss_Reader", MB_OK);
    }

    Ok(())
}
