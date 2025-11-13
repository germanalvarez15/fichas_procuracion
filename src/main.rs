mod models;
mod storage;
mod ui;

use iced::widget::container;
use iced::{Element, Length, Task, Theme};
use std::path::PathBuf;

use storage::JsonStorage;
use ui::fichas::FichaMessage;
use ui::FichasView;

fn main() -> iced::Result {
    iced::application(
        FichasProcuracionApp::title,
        FichasProcuracionApp::update,
        FichasProcuracionApp::view,
    )
    .theme(FichasProcuracionApp::theme)
    .run_with(FichasProcuracionApp::new)
}

#[derive(Debug, Clone)]
enum Message {
    Ficha(FichaMessage),
}

struct FichasProcuracionApp {
    storage: JsonStorage,
    fichas_view: FichasView,
}

impl FichasProcuracionApp {
    fn new() -> (Self, Task<Message>) {
        let mut storage = JsonStorage::new(PathBuf::from("datos_procuracion.json"));

        // Intentar cargar datos
        let _ = storage.cargar();

        // Inicializar la vista de fichas con los datos cargados
        let mut fichas_view = FichasView::new();
        fichas_view.actualizar_fichas(storage.obtener_fichas().clone());

        (
            Self {
                storage,
                fichas_view,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Fichas Procuración - Sistema de Gestión")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Ficha(ficha_msg) => {
                self.manejar_mensaje_ficha(ficha_msg);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let contenido = self.fichas_view.view().map(Message::Ficha);

        container(contenido)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

impl FichasProcuracionApp {
    fn manejar_mensaje_ficha(&mut self, mensaje: FichaMessage) {
        match mensaje {
            FichaMessage::NuevaFicha => {
                self.fichas_view.iniciar_nueva_ficha();
            }
            FichaMessage::EditarFicha(id) => {
                self.fichas_view.iniciar_edicion(id);
            }
            FichaMessage::EliminarFicha(id) => {
                let _ = self.storage.eliminar_ficha(id);
                self.fichas_view
                    .actualizar_fichas(self.storage.obtener_fichas().clone());
            }
            FichaMessage::TituloChanged(valor) => {
                self.fichas_view.titulo_input = valor;
            }
            FichaMessage::DescripcionChanged(valor) => {
                self.fichas_view.descripcion_input = valor;
            }
            FichaMessage::HojaChanged(hoja) => {
                self.fichas_view.hoja_input = hoja.clone();
                self.fichas_view.nueva_hoja_input = hoja;
            }
            FichaMessage::VerHistorial(ficha_id) => {
                self.fichas_view.ficha_historial_id = Some(ficha_id);
                self.fichas_view.nueva_hoja_input.clear();
            }
            FichaMessage::CerrarHistorial => {
                self.fichas_view.ficha_historial_id = None;
                self.fichas_view.nueva_hoja_input.clear();
            }
            FichaMessage::AgregarHoja(ficha_id) => {
                if !self.fichas_view.nueva_hoja_input.is_empty() {
                    if let Some(ficha) = self.storage.obtener_ficha_mut(ficha_id) {
                        ficha.agregar_hoja(self.fichas_view.nueva_hoja_input.clone());
                        let _ = self.storage.guardar();
                        self.fichas_view.nueva_hoja_input.clear();
                    }
                    self.fichas_view
                        .actualizar_fichas(self.storage.obtener_fichas().clone());
                }
            }
            FichaMessage::GuardarFicha => {
                if self.fichas_view.ficha_editando_id.is_some() {
                    // Editar ficha existente
                    if let Some(ficha) = self.fichas_view.obtener_ficha_editada() {
                        let _ = self.storage.actualizar_ficha(ficha);
                    }
                } else {
                    // Crear nueva ficha
                    if let Some(ficha) = self.fichas_view.obtener_ficha_nueva() {
                        let _ = self.storage.agregar_ficha(ficha);
                    }
                }
                self.fichas_view.cancelar_edicion();
                self.fichas_view
                    .actualizar_fichas(self.storage.obtener_fichas().clone());
            }
            FichaMessage::CancelarEdicion => {
                self.fichas_view.cancelar_edicion();
            }
        }
    }
}
