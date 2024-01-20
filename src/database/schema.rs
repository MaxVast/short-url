use diesel::prelude::*;

//Define Models
table! {
    short_urls {
        id -> Text,
        url -> Text,
        short_url -> Text,
    }
}

//struct table
#[derive(Queryable, Insertable)]
#[diesel(table_name = short_urls)]
pub struct ShortUrl {
    pub id: String,
    pub url: String,
    pub short_url: String,
}