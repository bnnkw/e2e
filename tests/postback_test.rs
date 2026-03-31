mod common;

use axum::{response::Html, routing::get, Router};
use e2e::e2e_yaml::driver::Driver;
use e2e::e2e_yaml::scenario::Scenarios;
use e2e::e2e_yaml::step::{AssertTarget, Attribute, Step};
use e2e::e2e_yaml::{E2eYaml, Window};
use indexmap::IndexMap;

fn minimal_e2e_yaml(driver: Driver) -> E2eYaml {
    E2eYaml {
        driver,
        vars: None,
        tasks: None,
        scenarios: Scenarios(IndexMap::new()),
    }
}

async fn get_handler() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html>
<body>
  <form method="post" action="/">
    <span id="result"></span>
    <button id="btn" type="submit">Submit</button>
  </form>
</body>
</html>"#,
    )
}

async fn post_handler() -> Html<&'static str> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    Html(
        r#"<!DOCTYPE html>
<html>
<body>
  <form method="post" action="/">
    <span id="result">Clicked</span>
    <button id="btn" type="submit">Submit</button>
  </form>
</body>
</html>"#,
    )
}

#[tokio::test]
async fn test_click_and_wait() {
    let app = Router::new().route("/", get(get_handler).post(post_handler));
    let server = tokio::spawn(common::server::start(3000, app));

    let driver_config = Driver {
        host: "localhost".to_string(),
        port: "6666".to_string(),
        headless: true,
        window: Window {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        },
    };
    let driver = driver_config.initialize().await.unwrap();
    let e2e_yaml = minimal_e2e_yaml(driver_config);

    let steps = vec![
        Step::Goto("http://localhost:3000".to_string()),
        Step::ClickAndWait {
            selector: "#btn".to_string(),
            timeout: 5000,
            interval: 200,
        },
        Step::AssertEq {
            expected: "Clicked".to_string(),
            target: AssertTarget::Element {
                selector: "#result".to_string(),
                attr: Attribute::Text,
            },
        },
    ];

    for step in &steps {
        step.run(&driver, &e2e_yaml).await.unwrap();
    }

    driver.quit().await.unwrap();
    server.abort();
}
