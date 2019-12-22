// Copyright 2019 tinaxd
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate watsontts_rs as tts;

fn question(q: &str) -> std::io::Result<String> {
    use std::io::{Write, BufRead};
    print!("{}", q);
    std::io::stdout().flush().unwrap();
    let mut r = std::io::BufReader::new(std::io::stdin());
    let mut buf = String::new();
    match r.read_line(&mut buf) {
        Ok(_) => Ok(buf.trim().to_string()),
        Err(e) => Err(e)
    }
}

fn main() -> std::io::Result<()>{
    let url = question("watson url? ")?;
    let apikey = question("apikey? ")?;
    let voice = question("voice name? ")?;
    let format = question("audio format? ")?;
    let text = question("text? ")?;

    let conn = tts::WatsonConnection::new(url, apikey);
    let query = tts::WatsonTTSQuery::new(text, voice, format);

    let file = std::fs::File::create("output")?;
    let mut writer = std::io::BufWriter::new(file);

    tts::tts_blocking(&conn, &query, &mut writer).expect("watson error");
    Ok(())
}