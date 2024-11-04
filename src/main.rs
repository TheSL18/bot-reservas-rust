use teloxide::{prelude::*, types::{InlineKeyboardButton, InlineKeyboardMarkup, BotCommand, ParseMode}, utils::command::BotCommands};
use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "Comandos disponibles:")]
enum Command {
    #[command(description = "Iniciar el bot.")]
    Start,
    #[command(description = "Mostrar ayuda.")]
    Help,
    #[command(description = "Reservar un espacio.")]
    Reservar,
    #[command(description = "Cancelar una reserva.")]
    Cancelar,
    #[command(description = "Consultar detalles de una reserva.")]
    Consultar,
}

// Estructura de categorías y espacios
fn espacios() -> HashMap<&'static str, Vec<&'static str>> {
    let mut espacios: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    
    espacios.insert("Auditorio", vec![
        "Auditorio B205-C202", 
        "Auditorio Escalonado Grande", 
        "Auditorio Escalonado Pequeño"
    ]);

    espacios.insert("Plazoleta", vec![
        "Plazoleta Jesus y Maria", 
        "Plazoleta RGH", 
        "Plazoleta de la Cruz", 
        "Plazoleta Verde", 
        "Plazoleta R Piso 9",
        "Hall Torre B-C"
    ]);

    espacios.insert("Sala de Juntas", vec![
        "Sala de Juntas", "Sala de Juntas RGH", "Sala de Juntas FCC", "Sala de Juntas FCE",
        "Sala de Juntas FING", "Sala de Juntas FEDU", "Sala de Juntas I+D", "Sala de Juntas FCHS", 
        "Coworking 1", "Coworking 2"
    ]);

    espacios
}

// Conexión a la base de datos
fn connect_db() -> PooledConn {
    dotenv().ok();
    let db_host = env::var("DB_HOST").unwrap();
    let db_user = env::var("DB_USER").unwrap();
    let db_passwd = env::var("DB_PASSWD").unwrap();
    let db_db = env::var("DB_DB").unwrap();

    let url = format!("mysql://{}:{}@{}/{}", db_user, db_passwd, db_host, db_db);
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let bot_token = env::var("BOT_TOKEN").expect("Token de bot no encontrado.");
    let bot = Bot::new(bot_token).auto_send();

    teloxide::commands_repl(bot.clone(), Command::ty(), move |cx, command| async move {
        match command {
            Command::Start => {
                cx.answer("¡Hola! Soy Recon, tu asistente de reservas en UNIMINUTO. Usa /reservar para comenzar.")
                    .await?;
            }
            Command::Help => {
                cx.answer("Comandos disponibles:\n/reservar - Para hacer una reserva\n/cancelar - Para cancelar una reserva\n/consultar - Para consultar una reserva.")
                    .await?;
            }
            Command::Reservar => {
                // Lógica para comenzar una reserva
                cx.answer("Por favor, proporciona tu nombre:").await?;
                // Aquí continúa la interacción paso a paso con el usuario
            }
            Command::Cancelar => {
                // Lógica para cancelar una reserva
                cx.answer("Funcionalidad de cancelación en desarrollo...").await?;
            }
            Command::Consultar => {
                // Lógica para consultar reservas
                cx.answer("Funcionalidad de consulta en desarrollo...").await?;
            }
        }
        Ok::<(), Box<dyn std::error::Error>>(())
    })
    .await;
}

