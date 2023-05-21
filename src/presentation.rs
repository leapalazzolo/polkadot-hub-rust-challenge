use fltk::{
    app::{self, App},
    button::Button,
    menu::Choice,
    prelude::{GroupExt, WidgetExt},
    window::DoubleWindow,
};

use fltk::{app::*, browser::*, enums::*, input::*, output::*, prelude::*, window::*};

const WIDGET_WIDTH: i32 = 120;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
    Save,
}

use crate::{service::HouseService, utils::APARTMENT};

pub struct GUI {
    app: App,
    wind: DoubleWindow,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    service: HouseService,
    filter_input: Input,
    list_browser: HoldBrowser,
    id_output: Output,
    street_input: Input,
    street_number_input: Input,
    street_floor_input: Input,
    postal_code_input: Input,
    surface_input: Input,
    bathrooms_input: Input,
    rooms_input: Input,
    kind_input: Choice,
    create_button: Button,
    update_button: Button,
    delete_button: Button,
    save_button: Button,
    message_output: Output,
}

impl GUI {
    pub fn new(mut house_service: HouseService) -> GUI {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        let wind = Window::default().with_label("CRUD");
        let (sender, receiver) = channel::<Message>();

        let filter_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
            .with_label("Id:");

        let list_browser = HoldBrowser::default()
            .with_pos(
                WIDGET_PADDING,
                filter_input.y() + filter_input.height() + WIDGET_PADDING,
            )
            .with_size(WIDGET_WIDTH * 5, WIDGET_HEIGHT * 12);

        let mut id_output = Output::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH,
                list_browser.y(),
            )
            .with_label("Id:");
        id_output.deactivate();

        let street_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&id_output, WIDGET_PADDING)
            .with_label("Calle:");

        let street_number_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&street_input, WIDGET_PADDING)
            .with_label("Número:");

        let postal_code_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&street_number_input, WIDGET_PADDING)
            .with_label("CP:");

        let surface_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&postal_code_input, WIDGET_PADDING)
            .with_label("Superfice:");

        let bathrooms_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&surface_input, WIDGET_PADDING)
            .with_label("Baños:");

        let rooms_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&bathrooms_input, WIDGET_PADDING)
            .with_label("Habitaciones:");

        let mut kind_input = Choice::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&rooms_input, WIDGET_PADDING)
            .with_label("Tipo:");

        let kinds = house_service
            .get_houses_kind()
            .unwrap_or_else(|_| panic!("Can't connect to the DB"));

        for k in kinds {
            kind_input.add_choice(&k.kind);
        }

        let mut street_floor_input = Input::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .below_of(&kind_input, WIDGET_PADDING)
            .with_label("Piso:");
        street_floor_input.deactivate();

        let create_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                WIDGET_PADDING,
                street_floor_input.y() + street_floor_input.height() + WIDGET_PADDING,
            )
            .with_label("Crear");

        let update_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&create_button, WIDGET_PADDING)
            .with_label("Modificar");

        let delete_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&update_button, WIDGET_PADDING)
            .with_label("Borrar");

        let save_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&delete_button, WIDGET_PADDING)
            .with_label("Guardar");

        let message_output = Output::default()
            .with_size(420, WIDGET_HEIGHT)
            .right_of(&save_button, WIDGET_PADDING * 3);
        // .with_label("Id:");
        // .with_pos(
        //     list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH,
        //     list_browser.y(),
        // );

        GUI {
            app: app,
            wind: wind,
            sender: sender,
            receiver: receiver,
            id_output: id_output,
            filter_input: filter_input,
            list_browser: list_browser,
            service: house_service,
            street_input,
            street_number_input,
            street_floor_input,
            postal_code_input,
            surface_input,
            bathrooms_input,
            rooms_input,
            kind_input,
            create_button: create_button,
            update_button: update_button,
            delete_button: delete_button,
            save_button: save_button,
            message_output: message_output,
        }
    }

    pub fn build(&mut self) {
        self.filter_input.set_trigger(CallbackTrigger::Changed);
        self.filter_input.emit(self.sender, Message::Filter);

        self.list_browser.emit(self.sender, Message::Select);

        self.create_button.emit(self.sender, Message::Create);

        self.update_button.emit(self.sender, Message::Update);
        self.update_button.deactivate();

        self.delete_button.emit(self.sender, Message::Delete);
        self.delete_button.deactivate();

        self.save_button.emit(self.sender, Message::Save);

        self.wind.set_size(
            self.street_input.x() + self.street_input.width() + WIDGET_PADDING,
            self.update_button.y() + self.update_button.height() + WIDGET_PADDING,
        );

        self.sender.send(Message::Filter);
    }

    fn clear_edit(&mut self) {
        self.street_input.set_value("");
        self.street_number_input.set_value("");
        self.street_floor_input.set_value("");
        self.postal_code_input.set_value("");
        self.surface_input.set_value("");
        self.bathrooms_input.set_value("");
        self.rooms_input.set_value("");
        self.kind_input.set_value(-1);
        self.id_output.set_value("");
        // self.message_output.set_value("");
    }

    pub fn show_message(&mut self, message: &str) {
        self.message_output.set_text_color(Color::Black);
        self.message_output.set_value(message);
        println!("{message}");
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();
        while self.app.wait() {
            if self.kind_input.value() == APARTMENT {
                self.street_floor_input.activate();
            } else {
                self.street_floor_input.set_value("");
                self.street_floor_input.deactivate();
            }
            match self.receiver.recv() {
                Some(Message::Create) => {
                    self.clear_edit();
                    self.show_message(&format!("Complete los campos"));
                    self.save_button.activate();
                    self.sender.send(Message::Filter);
                }
                Some(Message::Update) => {
                    if self.list_browser.value() > 0 {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();

                        match self
                            .service
                            .get_houses()
                            .unwrap()
                            .iter()
                            .filter(|h| h.to_string().eq_ignore_ascii_case(&text_selection))
                            .next()
                        {
                            Some(_) => {
                                let updated_house = self.service.update_house(
                                    &self.id_output.value(),
                                    &self.street_input.value(),
                                    &self.street_number_input.value(),
                                    &self.street_floor_input.value(),
                                    &self.postal_code_input.value(),
                                    &self.surface_input.value(),
                                    &self.bathrooms_input.value(),
                                    &self.rooms_input.value(),
                                    self.kind_input.value(),
                                );
                                if updated_house.is_err() {
                                    self.show_message(&format!(
                                        "Error actualizando el elemento #{}",
                                        self.id_output.value()
                                    ));
                                } else {
                                    self.show_message(&format!(
                                        "Elemento #{} actualizado",
                                        self.id_output.value()
                                    ));
                                }
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            }
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            }
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA MODIFICAR!!!");
                    }
                }
                Some(Message::Delete) => {
                    self.save_button.deactivate();
                    if self.list_browser.value() > 0 {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();
                        match self
                            .service
                            .get_houses()
                            .unwrap()
                            .iter()
                            .filter(|h| h.to_string().eq_ignore_ascii_case(&text_selection))
                            .next()
                        {
                            Some(house) => {
                                if self.service.delete_house(house.id).is_ok() {
                                    self.show_message(&format!(
                                        "Elemento #{} eliminado",
                                        self.id_output.value()
                                    ));
                                    self.clear_edit();
                                    self.sender.send(Message::Filter);
                                    self.sender.send(Message::Select);
                                } else {
                                    self.show_message(&format!(
                                        "Error eliminando el elemento {}",
                                        self.id_output.value()
                                    ));
                                }
                            }
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            }
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA ELIMINAR!!!");
                    }
                }
                Some(Message::Save) => {
                    let new_house = self.service.create_house(
                        &self.street_input.value(),
                        &self.street_number_input.value(),
                        &self.street_floor_input.value(),
                        &self.postal_code_input.value(),
                        &self.surface_input.value(),
                        &self.bathrooms_input.value(),
                        &self.rooms_input.value(),
                        self.kind_input.value(),
                    );
                    if new_house.is_err() {
                        self.show_message(&format!("Error guardando el nuevo elemento"));
                    } else {
                        self.show_message(&format!("Elemento nuevo guardado"));
                        self.clear_edit();
                        self.sender.send(Message::Filter);
                        self.sender.send(Message::Select);
                        self.save_button.deactivate();
                    }
                }
                Some(Message::Select) => {
                    self.save_button.activate();
                    if self.list_browser.value() == 0 {
                        self.update_button.deactivate();
                        self.delete_button.deactivate();
                    } else {
                        let text_selection =
                            self.list_browser.text(self.list_browser.value()).unwrap();

                        match self
                            .service
                            .get_houses()
                            .unwrap()
                            .iter()
                            .filter(|h| h.to_string().eq_ignore_ascii_case(&text_selection))
                            .next()
                        {
                            Some(house) => {
                                self.id_output.set_value(&house.id.to_string());
                                self.street_input.set_value(&house.street.to_string());
                                self.street_number_input
                                    .set_value(&house.street_number.to_string());
                                self.street_floor_input
                                    .set_value(&house.street_floor.to_string());
                                self.postal_code_input
                                    .set_value(&house.postal_code.to_string());
                                self.surface_input
                                    .set_value(&house.surface_square_meters.to_string());
                                self.bathrooms_input.set_value(&house.bathrooms.to_string());
                                self.rooms_input.set_value(&house.rooms.to_string());
                                self.kind_input.set_value(house.kind_id);
                                self.update_button.activate();
                                self.delete_button.activate();

                                self.show_message(&format!(
                                    "Elemento #{} seleccionado",
                                    self.id_output.value()
                                ));
                            }
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            }
                        }
                    }
                }
                Some(Message::Filter) => {
                    self.save_button.deactivate();
                    let prefix = self.filter_input.value().to_lowercase();
                    let filter_empty = prefix.trim().eq_ignore_ascii_case("");
                    self.list_browser.clear();
                    for (_, h) in self.service.get_houses().unwrap().iter().enumerate() {
                        if (h.id.to_string().contains(prefix.as_str()) && !filter_empty)
                            || (filter_empty)
                        {
                            let item = h.to_string();
                            self.list_browser.add(&item);
                        }
                    }
                    self.sender.send(Message::Select);
                }
                None => {}
            }
        }
    }
}
