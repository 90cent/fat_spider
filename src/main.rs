use std::{io::{BufReader, Result, Write, prelude},env};
use reqwest::{self, Url};
use tokio::{fs::File, io::AsyncWriteExt, runtime::{Runtime}};
use async_std;

mod helper;
use helper::{search::{google::*},filter::{filter_presets,filter_links}};

use crate::helper::search::google;


#[async_std::main]
async fn main() {
    let runtime = Runtime::new().unwrap();
    let args: Vec<String> = env::args().collect();
    let mut google_site_contents:Vec<String> = Vec::new();

    if args.len() <= 1 {
        println!("Please consider using this Syntax: [executable file] (search query) (Number of Sites that will be scraped).\n FatSpider:\n  Made by GEGAKE");
        return;
    }

    println!("{:#?}",args);

    let search = &args[1];
    let page = &args[2];
   
    let google_search_handle = runtime.spawn(google::search(search.into(),page.parse().unwrap()));
    let site_content = google_search_handle.await.unwrap();
    
    google_site_contents.push(site_content);
    
    
    runtime.block_on(async {
        let mut file = File::create("bruh4420.txt").await.unwrap();
        for site in filter_links(google_site_contents) {
            let site_bytes = &format!("{}\n",site).as_bytes().to_owned();

            file.write_all(site_bytes).await.unwrap();
            file.sync_all().await.unwrap();
        };
        println!("File Created");
        file.shutdown().await.unwrap();
    });
  
} 

fn start_leeching() {

}

fn start_search(search_provider: String,search_provider_arguments: Vec<String>) -> String   // Returns raw html content. // Jeder Provider hat eigene argumente, cringe wer keine hat. // Mögliche Provider: Pastebin, Google, Bing, Yandex, Baidu, Youtube
{
    return  "".into();
}




