//! Gestion IHM
//!
//! On utilise ici le [crate iced](https://iced.rs/) pour l'interface graphique
//!

use iced::widget::Text;
use iced::{executor, window, Application, Command, Element, Settings, Theme};

use crate::st2150::ST2150_MESSAGE_NUMBERS;
use crate::Context;
use crate::ST2150;

/// Structure pour initialiser l'IHM
/// Cette structure permet d'initialiser la structure `AppView` dans l'implémentation de `iced::Application`
#[derive(Default)]
pub struct AppSettings {
    /// Protocole ST2150 configuré avec le port série défini par l'utilisateur (voir main.rs)
    st2150: ST2150,
}

/// Structure pour l'IHM iced
pub struct AppView {
    /// Gestion du protocole ST2150 sur un port série défini
    st2150: ST2150,

    /// Contexte de l'utilisateur (informations gérées)
    context: Context,

    /// Numéro de message sélectionné
    message_num: u8,
}

/// Point d'entrée de l'IHM
pub fn run(st2150: ST2150) {
    // Création de la structure pour initialiser l'application
    let app_setting = AppSettings { st2150 };

    // Exécution de l'application IHM avec le runtime Iced
    // Ici on définit `flag` qui porte la sélection du port série et le protocole ST2150 associé
    let _ = AppView::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        flags: app_setting,
        ..Settings::default()
    });
}

/// Message (Command) pour les actions de l'utilisateur
#[derive(Debug, Clone)]
pub enum Message {
    SelectMessage(u8),
    DoVacation,
}

impl Application for AppView {
    type Executor = executor::Default;
    type Flags = AppSettings;
    type Message = Message;
    type Theme = Theme;

    fn new(flags: AppSettings) -> (AppView, Command<Self::Message>) {
        (
            AppView {
                st2150: flags.st2150,
                context: Context::default(),
                message_num: ST2150_MESSAGE_NUMBERS[0],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Simulateur Informatique Embarquée ALMA - ST2150".to_string()
    }

    /// Traitement des messages de l'application
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectMessage(_message_num) => Command::none(),
            Message::DoVacation => Command::none(),
        }
    }

    // Mise à jour affichage de l'application
    fn view(&self) -> Element<Message> {
        // Créez une colonne avec un texte affichant la valeur du compteur et un message
        Text::new(format!(
            "Hello form iced ! Message: {:02}",
            self.message_num
        ))
        .into()
    }
}
