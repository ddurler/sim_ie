//! Gestion IHM
//!
//! On utilise ici le [crate iced](https://iced.rs/) pour l'interface graphique
//!

mod message00;
mod message10;
mod messages;

use message00::Message00;
use message10::Message10;
use messages::CommonMessageTrait;

use iced::widget::{column, horizontal_rule, row, Button, Column, Row, Text};
use iced::{executor, theme, window, Application, Command, Element, Settings, Theme};

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

    /// Message sélectionné
    dyn_message: Box<dyn CommonMessageTrait>,
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

/// Accès au `CommonMessageTrait` des différents messages gérés
fn get_dyn_message(message_num: u8) -> Box<dyn CommonMessageTrait> {
    match message_num {
        0 => Box::<Message00>::default(),
        10 => Box::<Message10>::default(),
        _ => panic!("Numéro de message non géré {message_num}"),
    }
}

/// Message (Command) pour les actions de l'utilisateur
#[derive(Debug, Clone)]
pub enum Message {
    SelectionMessage(u8),
    DoMessageVacation,
}

impl AppView {
    /// Sélection du message courant
    fn set_current_message_num(&mut self, message_num: u8) {
        self.dyn_message = get_dyn_message(message_num);
    }

    /// Header affichage
    fn str_header(&self) -> String {
        let availability =
            match ST2150::message_availability(&self.context, self.dyn_message.message_num()) {
                Ok(_) => "Prêt...".to_string(),
                Err(e) => format!("{e}"),
            };
        format!(
            "Message '{:02}' sur le port {} : {}",
            self.dyn_message.message_num(),
            self.st2150.port.name,
            availability
        )
    }

    /// Contenu du header en affichage
    pub fn view_header(&self) -> Element<Message> {
        Text::new(self.str_header()).into()
    }

    /// Zone pour sélection du message courant
    pub fn body_message_selection(&self) -> Element<Message> {
        // Numéro de message actuellement sélectionné
        let cur_message_num = self.dyn_message.message_num();

        let mut col = Column::new();

        for message_num in ST2150_MESSAGE_NUMBERS {
            let dyn_message = get_dyn_message(*message_num);
            let text: Text = Text::new(format!("{:02} {}", message_num, dyn_message.str_message())).size(12);
            let btn = if *message_num == cur_message_num {
                // C'est le numéro de message actuellement sélectionné
                Button::new(text).on_press(Message::SelectionMessage(*message_num))
            } else {
                // Numéro de message non sélectionné. On l'affiche en noir sur fond gris
                Button::new(text)
                    .on_press(Message::SelectionMessage(*message_num))
                    .style(theme::Button::Secondary)
            };
            col = col.push(btn);
        }

        col.into()
    }

    /// Contenu du footer en affichage
    pub fn view_footer(&self) -> Element<Message> {
        let col = Column::new();

        // Dernière requête
        let my_str = if self.st2150.last_error.is_empty() {
            "(Pas de requête)".to_string()
        } else {
            format!("Requête : {:?}", self.st2150.last_req)
        };
        let txt: Text = Text::new(my_str.to_string());
        let col = col.push(txt);

        // Dernière réponse
        let my_str = if self.st2150.last_rep.is_empty() {
            "(Pas de réponse)".to_string()
        } else {
            format!("Réponse : {:?}", self.st2150.last_rep)
        };
        let txt: Text = Text::new(my_str.to_string());
        let col = col.push(txt);

        // Dernière erreur
        let my_str = if self.st2150.last_error.is_empty() {
            "(Pas d'erreur)".to_string()
        } else {
            format!("/!\\ ERREUR : {} /!\\", self.st2150.last_error)
        };
        let txt: Text = Text::new(my_str.to_string());
        let col = col.push(txt);

        col.into()
    }
}

impl Application for AppView {
    type Executor = executor::Default;
    type Flags = AppSettings;
    type Message = Message;
    type Theme = Theme;

    /// Constructeur de `AppView` (sur la base de `AppSettings`)
    fn new(flags: AppSettings) -> (AppView, Command<Self::Message>) {
        (
            AppView {
                st2150: flags.st2150,
                context: Context::default(),
                dyn_message: Box::<Message00>::default(), // Message00 par défaut
            },
            Command::none(),
        )
    }

    /// Titre de l'application
    fn title(&self) -> String {
        "Simulateur Informatique Embarquée ALMA - ST2150".to_string()
    }

    /// Traitement des messages de l'application
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectionMessage(message_num) => self.set_current_message_num(message_num),
            Message::DoMessageVacation => (),
        }
        Command::none()
    }

    /// Mise à jour affichage de l'application
    fn view(&self) -> Element<Message> {
        column![
            // Un header de status sur la 1er ligne
            self.view_header(),
            // Le corps du simulateur
            horizontal_rule(10),
            self.body_message_selection(),
            // Un footer avec les traces dernières requête/réponse/erreur
            horizontal_rule(10),
            self.view_footer(),
        ]
        .into()
    }
}
