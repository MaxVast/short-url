use askama::Template;

// Structure for context template
#[derive(Template)]
#[template(path = "index.html")]
pub struct HomeTemplate {}


#[derive(Template)]
#[template(path = "upload_template.html")]
pub struct UploadTemplate {
    pub image: String,
}

#[derive(Template)]
#[template(path = "upload_csv.html")]
pub struct UploadCsvTemplate {}