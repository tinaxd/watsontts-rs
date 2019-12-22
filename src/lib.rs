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

extern crate reqwest;
extern crate url;

#[derive(Debug, Clone)]
pub enum TTSError {
    ConnectionError(String),
    WatsonError
}

impl From<reqwest::Error> for TTSError {
    fn from(t: reqwest::Error) -> TTSError {
        TTSError::ConnectionError(t.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct WatsonConnection {
    url: String,
    apikey: String
}

impl WatsonConnection {
    pub fn new<T: Into<String>>(url: T, apikey: T) -> Self{
        WatsonConnection {url: url.into(), apikey: apikey.into()}
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}

#[derive(Debug, Clone)]
pub struct WatsonTTSQuery {
    input_text: String,
    voice: String,
    audio_format: String
}

impl WatsonTTSQuery {
    pub fn new<T: Into<String>>(input_text: T, voice: T, audio_format: T) -> Self {
        WatsonTTSQuery {
            input_text: input_text.into(),
            voice: voice.into(),
            audio_format: audio_format.into()
        }
    }
}

pub fn tts_blocking<W: std::io::Write>(conn: &WatsonConnection, query: &WatsonTTSQuery, writer: &mut W) -> Result<(), TTSError> {
    let mut body = std::collections::HashMap::new();
    body.insert("text", &query.input_text);

    let client = reqwest::blocking::Client::new();
    let mut res = client
                    .post(&format!("{}/v1/synthesize?voice={}", conn.get_url(), query.voice))
                    .basic_auth("apikey", Some(&conn.apikey))
                    .header("Accept", &query.audio_format)
                    .json(&body)
                    .send()?;
    res.copy_to(writer)?;
    if res.status().is_success() { Ok(()) } else { Err(TTSError::WatsonError) }
}