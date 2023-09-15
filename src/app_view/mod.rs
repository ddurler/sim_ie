//! Gestion IHM
//!
//! On utilise ici le [crate iced](https://iced.rs/) pour l'interface graphique
//!

use std::collections::HashMap;

mod input_infos;
mod show_infos;

use super::APP_VERSION;

use iced::widget::{checkbox, column, container, horizontal_rule, row, vertical_rule};
use iced::widget::{Button, Column, Row, Text};
use iced::{executor, theme, window, Length};
use iced::{Application, Command, Element, Settings, Theme};

use crate::context::IdInfo;
use crate::st2150::messages::{
    get_dyn_message, message00::Message00, CommonMessageTrait, ST2150_MESSAGE_NUMBERS,
};
use crate::st2150::Edition2150;
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

    /// Editions de la ST2150 à afficher
    editions_st2150: HashMap<Edition2150, bool>,
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
            min_size: Some((240, 50)),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        default_text_size: 12.0,
        flags: app_setting,
        ..Settings::default()
    });
}

/// Message (Command) pour les actions de l'utilisateur
#[derive(Debug, Clone)]
pub enum Message {
    SelectionMessageST2150(u8),
    DoMessageVacation(u8),
    InputInfo(String, IdInfo),
    SelectionEditionST2150(Edition2150, bool),
}

impl AppView {
    /// Sélection du message courant
    fn set_current_message_num(&mut self, message_num: u8) {
        self.dyn_message = get_dyn_message(message_num);
    }

    /// Indique si une édition de la ST2150 est visible
    fn is_edition_st2150_visible(&self, edition: Edition2150) -> bool {
        match self.editions_st2150.get(&edition) {
            Some(value) => *value,
            None => false,
        }
    }

    /// Mise à jour de la visibilité d'une édition de la ST2150
    fn set_edition_st2150_visible(&mut self, edition: Edition2150, value: bool) {
        self.editions_st2150.insert(edition, value);
    }

    /// Liste des numéros de messages visibles selon la sélection de(s) édition(s) sélectionnée(s)
    /// en incluant les messages de la pré-sélection d'office
    fn list_visible_message_nums(&self, pre_selection: &[u8]) -> Vec<u8> {
        let mut visible_message_nums = vec![];

        for message_num in ST2150_MESSAGE_NUMBERS {
            if pre_selection.contains(message_num)
                || self.is_edition_st2150_visible(get_dyn_message(*message_num).edition_st2150())
            {
                visible_message_nums.push(*message_num);
            }
        }

        visible_message_nums
    }

    /// Zone pour sélection du message courant
    pub fn body_message_selection(&self) -> Element<Message> {
        // Numéro de message actuellement sélectionné
        let cur_message_num = self.dyn_message.message_num();

        // Liste des numéros de messages visibles
        let visible_message_nums = self.list_visible_message_nums(&[cur_message_num]);

        let mut row = Row::new();

        let mut col1 = Column::new();
        let mut col2 = Column::new();

        for (n, message_num) in visible_message_nums.iter().enumerate() {
            let text: Text = Text::new(format!(
                "{:02} {}",
                message_num,
                get_dyn_message(*message_num).message_str()
            ));
            let btn = if *message_num == cur_message_num {
                // C'est le numéro de message actuellement sélectionné
                Button::new(text).on_press(Message::SelectionMessageST2150(*message_num))
            } else {
                // Numéro de message non sélectionné. On l'affiche en noir sur fond gris
                Button::new(text)
                    .on_press(Message::SelectionMessageST2150(*message_num))
                    .style(theme::Button::Secondary)
            };

            // Si le nombre de messages à afficher est plus grand que la moitié de tous les messages possibles...
            if visible_message_nums.len() > ST2150_MESSAGE_NUMBERS.len() / 2
            // ...et qu'on est sur un indice impair d'affichage...
            && n % 2 == 1
            {
                // ...affichage sur 2 colonnes...
                col2 = col2.push(btn);
            } else {
                // ...sinon qu'une seule colonne
                col1 = col1.push(btn);
            }
        }

        row = row.push(col1).push(col2).spacing(10);

        row.into()
    }

    /// Informations pour la requête courante
    pub fn view_request(&self) -> Element<Message> {
        let id_infos = self.dyn_message.id_infos_request();

        let mut col = Column::new();

        if id_infos.is_empty() {
            let txt = Text::new("(Pas de champ)");
            col = col.push(txt);
        } else {
            for id_info in id_infos {
                let w = input_infos::input_info(&self.context, id_info);
                col = col.push(w);
            }
        }

        col.into()
    }

    /// Informations pour la réponse courante
    pub fn view_response(&self) -> Element<Message> {
        // Si la réponse contient NACK à true, on n'affiche que cette info
        fn is_nack(context: &Context, id_infos: &[IdInfo]) -> bool {
            if id_infos.contains(&IdInfo::Nack) {
                if let Some(value) = context.get_info_bool(IdInfo::Nack) {
                    return value;
                }
            }
            false
        }

        let id_infos = self.dyn_message.id_infos_response();

        let mut col = Column::new();

        if id_infos.is_empty() {
            let txt = Text::new("(Pas d'information)");
            col = col.push(txt);
        } else if is_nack(&self.context, &id_infos) {
            let txt = Text::new("REPONSE : NACK !!!");
            col = col.push(txt);
        } else if !self.st2150.last_error.is_empty() {
            let txt = format!("ERREUR : {} !!!", self.st2150.last_error);
            let txt = Text::new(txt);
            col = col.push(txt);
        } else {
            for id_info in id_infos {
                let w = show_infos::show_info(&self.context, id_info);
                col = col.push(w);
            }
        }

        col.into()
    }

    /// Zone avec bouton action selon le contexte
    pub fn view_do_vacation(&self) -> Element<Message> {
        let mut row = Row::new();

        /* Disponibilité ? */
        match ST2150::message_availability(&self.context, self.dyn_message.message_num()) {
            Ok(_) => {
                // Bouton pour exécuter cette commande
                let txt_do_it = format!(
                    "Run Message {:02} ({}) sur le port {}",
                    self.dyn_message.message_num(),
                    self.dyn_message.message_str(),
                    self.st2150.port.name,
                );
                let txt_do_it: Text = Text::new(txt_do_it);
                let btn_do_it = Button::new(txt_do_it)
                    .on_press(Message::DoMessageVacation(self.dyn_message.message_num()));
                row = row.push(btn_do_it);
            }
            Err(e) => {
                // Info de cette commande (indisponible)
                let txt_info = format!(
                    "Message '{:02}' sur le port {} : {}",
                    self.dyn_message.message_num(),
                    self.st2150.port.name,
                    e
                );
                let txt_info: Text = Text::new(txt_info);
                row = row.push(txt_info);
            }
        };

        row.into()
    }

    /// Zone avec sélection des éditions de la ST2150 à afficher
    pub fn view_edition_st2150(&self) -> Element<Message> {
        row![
            Text::new("Editions ST2150 : "),
            checkbox(
                "A",
                self.is_edition_st2150_visible(Edition2150::A),
                |value| Message::SelectionEditionST2150(Edition2150::A, value)
            ),
            checkbox(
                "B",
                self.is_edition_st2150_visible(Edition2150::B),
                |value| Message::SelectionEditionST2150(Edition2150::B, value)
            ),
            checkbox(
                "C",
                self.is_edition_st2150_visible(Edition2150::C),
                |value| Message::SelectionEditionST2150(Edition2150::C, value)
            ),
        ]
        .spacing(25)
        .into()
    }

    /// Zone avec les traces / erreur de la dernière vacation
    pub fn view_vacation(&self) -> Element<Message> {
        let col = Column::new();

        // Dernière requête
        let my_str = if self.st2150.last_req.is_empty() {
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
        // Toutes les révisions visibles par défaut
        let mut editions_st2150 = HashMap::new();
        editions_st2150.insert(Edition2150::A, true);
        editions_st2150.insert(Edition2150::B, true);
        editions_st2150.insert(Edition2150::C, true);

        // Objet AppView pour l'IHM
        (
            AppView {
                st2150: flags.st2150,
                context: Context::default(),
                dyn_message: Box::<Message00>::default(), // Message00 par défaut
                editions_st2150,
            },
            Command::none(),
        )
    }

    /// Titre de l'application
    fn title(&self) -> String {
        format!("Simulateur v{APP_VERSION} - Informatique Embarquée ALMA")
    }

    /// Traitement des messages de l'application
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectionMessageST2150(message_num) => {
                self.set_current_message_num(message_num);
                self.st2150.last_req = vec![];
                self.st2150.last_rep = vec![];
                self.st2150.last_error = String::new();
                Command::none()
            }
            Message::DoMessageVacation(message_num) => {
                let _ = self
                    .st2150
                    .do_message_vacation(&mut self.context, message_num);
                Command::none()
            }
            Message::InputInfo(input, id_info) => {
                input_infos::callback_input_info(&mut self.context, &input, id_info);
                Command::none()
            }
            Message::SelectionEditionST2150(edition, value) => {
                self.set_edition_st2150_visible(edition, value);
                Command::none()
            }
        }
    }

    /// Mise à jour affichage de l'application
    fn view(&self) -> Element<Message> {
        column![
            container(row![
                // Sélection du message courant
                self.body_message_selection(),
                // Partie 'requête' du message courant
                vertical_rule(10),
                self.view_request(),
                // Partie 'réponse' du message courant
                vertical_rule(10),
                self.view_response(),
            ])
            .height(Length::Fill),
            // Status/Vacation selon action
            horizontal_rule(10),
            row![self.view_do_vacation(), self.view_edition_st2150(),].spacing(10),
            // Trace dernières requête/réponse/erreur
            horizontal_rule(10),
            self.view_vacation(),
        ]
        .into()
    }
}
