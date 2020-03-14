use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fs;

#[command]
fn read(context: &mut Context, message: &Message) -> CommandResult {
    println!("Message:\n{:#?}", message);
    let mut response = String::new();
    for attachment in &message.attachments {
        let content = match attachment.download() {
            Ok(content) => content,
            Err(why) => {
                println!("Error downloading attachment: {:?}", why);
                let _ = message
                    .channel_id
                    .say(&context.http, "Error downloading attachment");

                return Ok(());
            }
        };

        let filename = get_filename(&attachment.filename);
        if filename.eq_ignore_ascii_case("") {
            response = String::from("Sorry, can't read that");
        } else {
            let mut file = match File::create(&filename) {
                Ok(file) => file,
                Err(why) => {
                    println!("Error creating file: {:?}", why);
                    let _ = message.channel_id.say(&context.http, "Error creating file");

                    return Ok(());
                }
            };
            if let Err(why) = file.write(&content) {
                println!("Error writing to file: {:?}", why);

                return Ok(());
            }
            response.push_str("The image contains the following text:\n\n");
            response.push_str(&get_response(&filename));
            fs::remove_file(&filename);
        }
    }
    let _ = message.channel_id.say(&context.http, response);

    Ok(())
}

fn get_filename(text: &str) -> String {
    let extensions = vec!["jpg", "png", "gif", "jpeg", "bmp", "tiff"];
    let vector: Vec<&str> = text.split(".").collect();
    if vector.len() > 1 {
        let extension: &str = vector[vector.len() - 1];
        if extensions.contains(&extension) {
            return format!("foo.{}", vector[vector.len() - 1]);
        }
    }

    String::from("")
}

fn get_response(filename: &str) -> String {
    let mut api = leptess::tesseract::TessApi::new(Some("./tessdata"), "eng").unwrap();
    let mut pix = leptess::leptonica::pix_read(Path::new(filename)).unwrap();
    api.set_image(&pix);

    let text = api.get_utf8_text();
    text.unwrap()
}
