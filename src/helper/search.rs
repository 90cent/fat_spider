use reqwest;

pub mod google {
    pub async fn search(search_query: String,pages_to_search: i32) -> String {    // Returns Site content //Man kann einfach die normalen seiten von google bei pages on search eingeben lol // 10000 Pages = 100 Pages on gulu gulu level
        let dorkses = ["intext:".to_string(),"intitle:".into()];
        let mut content: String = "".into();
    
        let pages = &pages_to_search * 10;
        let mut page = 0;
    
        while(page < pages) {
            for dorks in &dorkses {
                let search: String = format!("https://www.google.com/search?q={}\"{}\"&start={}",dorks,search_query,&page).into();
                let web_client = reqwest::Client::builder()
                    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
                    .redirect(reqwest::redirect::Policy::limited(0))
                   .build().unwrap();   
        
                let body = match web_client.get(reqwest::Url::parse(&search).unwrap()).send().await {
                    Ok(response) => {
                        let p = &page / 10;
                        println!("Finished:\nSearch Prefix: {}\nQuery: {}\nPage: {}",&dorks,&search_query,&p);
                        response
                    },
                    Err(error) => return format!("404 Error:{:?}",error.status()),
                };
               content = body.text().await.unwrap();

               page += 10;
            }
        }
        return content;
    }
}


mod pastebin {
    pub fn search() {
        
    }
}


mod bing {

}


mod yandex {

}

mod duckduckgo {

}

// ... Baidu,YouTube,...