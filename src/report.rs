use tera::{Context, Tera};

pub fn generate_report_as_html(total_requests: u64, rps: f64) -> String {
    let mut html_context = Context::new();
    html_context.insert("total_requests", &total_requests);
    html_context.insert("rps", &rps);

    let tera = Tera::new("templates/**/*.html").expect("Unable to generate as HTML file");
    tera.render("results.html", &html_context).unwrap()
}
