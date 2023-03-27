mod structs;

/// Fetches product details from aliexpress.
///
/// # Example
///
/// ```
/// let product_details: Product = get_product_details("1005005115304061").await.unwrap();
/// ```
///
/// You can get the product id from the url: https://aliexpress.com/item/[ID].html
pub async fn get_product_details(product_id: &str) -> Result<structs::Product, &str> {
    let page = match get_aliexpress_html(product_id).await {
        Err(e) => return Err(e),
        Ok(page) => page,
    };
    let run_params_string = match filter_run_params(page) {
        Ok(params) => params,
        Err(_) => return Err("Failed to parse product page"),
    };

    let run_params: structs::Product = serde_json::from_str(&run_params_string).unwrap();

    return Ok(run_params);
}

fn filter_run_params(page: String) -> Result<String, ()> {
    const END_PADDING: usize = 11;
    let mut res = page.as_str();

    res = res.split("window.runParams = ").collect::<Vec<&str>>()[1];
    res = res.split("};").collect::<Vec<&str>>()[0];
    let temp_res = res.to_owned() + "}";
    res = temp_res.as_str();
    let res_filter = res
        .lines()
        .filter(|line| {
            !line.contains("csrfToken")
                && !line.contains("abVersion")
                && !line.contains("abtestMap")
        })
        .collect::<Vec<&str>>();

    let joined_res_filter = res_filter.join("\n");
    res = joined_res_filter.as_str();
    let temp_res = res.replace("data:", "");
    res = temp_res.as_str();

    res = &res[1..res.len() - END_PADDING];

    return Ok(res.to_owned());
}

async fn get_aliexpress_html(product_id: &str) -> Result<String, &str> {
    let res_result =
        reqwest::get(format!("https://www.aliexpress.us/item/{product_id}.html")).await;

    if let Err(_) = res_result {
        return Err("Failed to fetch product page");
    } else if let Ok(res) = res_result {
        if res.status() != 200 {
            return Err("Failed to fetch product page");
        }

        let text_result = res.text().await;

        if let Err(_) = text_result {
            return Err("Failed to fetch product page");
        } else if let Ok(text) = text_result {
            if text.contains("Sorry, we can't find that page") {
                return Err("Product page not found");
            }

            return Ok(text);
        }
    }

    return Ok(String::new());
}
