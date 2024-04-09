use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::format;
use std::panic::resume_unwind;

#[derive(Debug)]
pub enum sending {
    /// # Enviar a mensagem e o id que vai receber
    /// * `mensagem`: A mensagem a ser enviada.
    /// * `chat_id`: O ID do chat para o qual a mensagem será enviada.
    /// ```
    /// EnviarMensagem(Mensagem, Chat_id)
    /// ```
    EnviarMensagem(String, i64),
    /// # Apagar a mensagem
    /// * `chat_id`: O ID do chat de onde a mensagem será apagada.
    /// * `id_mensagem`: O ID da mensagem a ser apagada.
    /// ```
    /// APagaMensagem(Chat_id,IdDaMensagem)
    /// ```
    APagaMensagem(String, i64),
    /// # Responder a mensagem
    /// * `chat_id`: O ID do chat onde a mensagem está.
    /// * `mensagem`: A mensagem de resposta.
    /// * `id_mensagem`: O ID da mensagem à qual está sendo respondida.
    /// ```
    /// ReplyToMensagem(Chat_id , Message , IdMessage)
    /// ```
    RePlyTOMensagem(String, String, i64),
    /// # Editar a mensagem
    /// * `chat_id`: O ID do chat onde a mensagem está.
    /// * `id_mensagem`: O ID da mensagem a ser editada.
    /// * `nova_mensagem`: A nova mensagem a ser exibida.

    /// ```
    ///  EditarMensagem(ChatId, Idmessage, NewMessage)
    /// ```
    EditarMensagem(String, i64, String),
    /// # enviar foto
    /// * `chat_id`: O ID do chat onde a foto será enviada.
    /// * `bytes_da_foto`: Os bytes da foto a ser enviada.
    /// * `filename`: O nome do arquivo da foto.
    /// ```
    ///  EnviarFoto(ChatId, Bytes, Filename)
    /// ```
    EnviarFoto(String, Vec<u8>, String),
    /// # Enviar foto com Mensagem
    /// * `chat_id`: O ID do chat onde a foto será enviada.
    /// * `bytes_da_foto`: Os bytes da foto a ser enviada.
    /// * `filename`: O nome do arquivo da foto.
    /// * `mensagem_caption`: A legenda da foto.
    /// ```
    /// EnviarFotoCaption(ChatId, Bytes, Filename, MensagemCaption)
    /// ```
    EnviarFotoCaption(String, Vec<u8>, String, String),
}

impl sending {
    pub fn send<T: AsRef<str>>(&self, api: T) -> Option<i64> {
        let ConfigUrl = format!("https://api.telegram.org/bot{}", api.as_ref());
        match self {
            sending::EditarMensagem(Chat_id, id_message, NovaMensagem) => RequestGet(format!(
                "{ConfigUrl}/editMessageText?chat_id={}&message_id={}&parse_mode=Markdown&text={}",
                Chat_id, id_message, NovaMensagem
            )),
            sending::EnviarMensagem(EnviarMensagem, Chat_id) => RequestGet(format!(
                "{ConfigUrl}/sendMessage?chat_id={}&parse_mode=Markdown&text={}",
                Chat_id, EnviarMensagem
            )),
            sending::APagaMensagem(Chat_id , IdDaMensagem) => {
                    RequestGet(format!(
                        "{ConfigUrl}/deleteMessage?chat_id={}&message_id={}",
                        Chat_id, IdDaMensagem
                    ))
            },
            sending::RePlyTOMensagem(Chat_id,Mensagem, IdReplyMessage) =>{

                RequestGet(format!(
                    "{ConfigUrl}/sendMessage?chat_id={}&parse_mode=Markdown&text={}&reply_to_message_id={}",
                    Chat_id, Mensagem , IdReplyMessage
                ))

            },

            sending::EnviarFoto(Chat_id, photo, filename) => {
                sendphoto(format!(
                    "{ConfigUrl}/sendPhoto?chat_id={}",
                    Chat_id
                ), photo ,filename)

            }
            sending::EnviarFotoCaption(ChatId, BytesPhoto, FilenamePhoto, Caption) =>{

                sendphoto(format!(
                    "{ConfigUrl}/sendPhoto?chat_id={}&parse_mode=Markdown&caption={}",
                    ChatId, Caption
                ), BytesPhoto, FilenamePhoto)


            }
            _ => None,
        }
    }
}

fn RequestGet<T: AsRef<str>>(url: T) -> Option<i64> {
    let mut HttpBuilder = Client::new();
    if let Ok(Response) = HttpBuilder.get(url.as_ref()).send() {
        if let Ok(jsonFormat) = serde_json::from_str::<Value>(
            &Response
                .text()
                .unwrap_or(String::from("Error ao fazer parse"))
                .as_str(),
        ) {
            return Some(jsonFormat["result"]["message_id"].as_i64().unwrap_or(0));
        } else {
            return None;
        }
    }

    return None;
}

fn sendphoto<T: AsRef<str>>(url: T, photo_bytes: &[u8], filename: &T) -> Option<i64> {
    println!("{}", url.as_ref());
    let http_post = Client::new();
    let postinfo = Form::new().part(
        "photo",
        Part::bytes(photo_bytes.to_vec()).file_name(filename.as_ref().to_owned()),
    );

    if let Ok(Response) = http_post.post(url.as_ref()).multipart(postinfo).send() {
        if let Ok(jsonFormat) = serde_json::from_str::<Value>(
            &Response
                .text()
                .unwrap_or(String::from("Error ao fazer parse"))
                .as_str(),
        ) {
            return Some(jsonFormat["result"]["message_id"].as_i64().unwrap_or(0));
        } else {
            return None;
        }
    }
    None
}
