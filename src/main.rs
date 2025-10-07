use chrome_driver_rs::ensure_latest_driver;
use serde::Serialize;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{thread, fs};
use thirtyfour::prelude::*;

#[derive(Serialize, Debug)]
struct Product {
    name: String,
    price: String,
    image: String,
    description: String,
    link: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // ✅ Automatically download and use the latest ChromeDriver
    let info = ensure_latest_driver("./driver").await.unwrap();

    // ✅ Start ChromeDriver
    Command::new(&info.driver_path)
        .args(["--port=9515"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("❌ Failed to start chromedriver");

    thread::sleep(Duration::from_secs(3));

    // ✅ Launch browser
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--headless")?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-gpu")?;
    caps.add_arg("--window-size=1280,900")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // ✅ Open the target page
    driver.goto("https://web-scraping.dev/products").await?;

    // ✅ Wait until all product blocks are loaded
    driver
        .query(By::Css("div.row.product"))
        .wait(Duration::from_secs(10), Duration::from_millis(500))
        .all()
        .await?;

    let products = driver.find_all(By::Css("div.row.product")).await?;
    let mut all_products: Vec<Product> = Vec::new();

    for product in products {
        let name = product
            .find(By::Css("h3.mb-0 a"))
            .await?
            .text()
            .await
            .unwrap_or_default();

        let price = product
            .find(By::Css("div.price"))
            .await?
            .text()
            .await
            .unwrap_or_default();

        let description = product
            .find(By::Css("div.short-description"))
            .await?
            .text()
            .await
            .unwrap_or_default();

        let image = product
            .find(By::Css("div.thumbnail img"))
            .await?
            .attr("src")
            .await?
            .unwrap_or_default();

        let link = product
            .find(By::Css("h3.mb-0 a"))
            .await?
            .attr("href")
            .await?
            .unwrap_or_default();

        all_products.push(Product {
            name,
            price,
            description,
            image,
            link,
        });
    }

    driver.quit().await?;

    // ✅ Save results to a JSON file
    let json = serde_json::to_string_pretty(&all_products).unwrap();
    fs::write("products.json", json).unwrap();

    println!(
        "✅ Scraping complete! {} products saved to products.json.",
        all_products.len()
    );
    Ok(())
}
