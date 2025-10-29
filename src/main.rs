mod models;
mod storage;
mod ui;

use iced::widget::container;
use iced::{Element, Length, Task, Theme};
use std::path::PathBuf;
use uuid::Uuid;

use storage::JsonStorage;
use ui::expedientes::ExpedienteMessage;
use ui::fichas::FichaMessage;
use ui::{ExpedientesView, FichasView};

fn main() -> iced::Result {
    iced::application(
        "Fichas Procuración - Sistema de Gestión",
        FichasProcuracionApp::update,
        FichasProcuracionApp::view,
    )
    .theme(FichasProcuracionApp::theme)
    .run_with(FichasProcuracionApp::new)
}

#[derive(Debug, Clone)]
enum Vista {
    Expedientes,
    Fichas(Uuid), // UUID del expediente seleccionado
}

#[derive(Debug, Clone)]
enum Message {
    Expediente(ExpedienteMessage),
    Ficha(FichaMessage),
    Cargado(Result<(), String>),
}

struct FichasProcuracionApp {
    storage: JsonStorage,
    vista_actual: Vista,
    expedientes_view: ExpedientesView,
    fichas_view: FichasView,
}

impl FichasProcuracionApp {
    fn new() -> (Self, Task<Message>) {
        let mut storage = JsonStorage::new(PathBuf::from("datos_procuracion.json"));

        // Intentar cargar datos
        let _ = storage.cargar();

        let mut expedientes_view = ExpedientesView::new();
        expedientes_view.actualizar_expedientes(storage.obtener_expedientes().clone());

        (
            Self {
                storage,
                vista_actual: Vista::Expedientes,
                expedientes_view,
                fichas_view: FichasView::new(),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Fichas Procuración - Sistema de Gestión")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Expediente(exp_msg) => {
                self.manejar_mensaje_expediente(exp_msg);
            }
            Message::Ficha(ficha_msg) => {
                self.manejar_mensaje_ficha(ficha_msg);
            }
            Message::Cargado(_) => {}
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let contenido = match &self.vista_actual {
            Vista::Expedientes => self.expedientes_view.view().map(Message::Expediente),
            Vista::Fichas(_) => self.fichas_view.view().map(Message::Ficha),
        };

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
    fn manejar_mensaje_expediente(&mut self, mensaje: ExpedienteMessage) {
        match mensaje {
            ExpedienteMessage::NuevoExpediente => {
                self.expedientes_view.iniciar_nuevo_expediente();
            }
            ExpedienteMessage::EditarExpediente(id) => {
                self.expedientes_view.iniciar_edicion(id);
            }
            ExpedienteMessage::EliminarExpediente(id) => {
                let _ = self.storage.eliminar_expediente(id);
                self.expedientes_view
                    .actualizar_expedientes(self.storage.obtener_expedientes().clone());
            }
            ExpedienteMessage::SeleccionarExpediente(id) => {
                if let Some(exp) = self.storage.obtener_expediente(id) {
                    self.fichas_view
                        .actualizar_fichas(exp.fichas.clone(), exp.nombre.clone());
                    self.vista_actual = Vista::Fichas(id);
                }
            }
            ExpedienteMessage::NumeroChanged(valor) => {
                self.expedientes_view.numero_input = valor;
            }
            ExpedienteMessage::NombreChanged(valor) => {
                self.expedientes_view.nombre_input = valor;
            }
            ExpedienteMessage::DescripcionChanged(valor) => {
                self.expedientes_view.descripcion_input = valor;
            }
            ExpedienteMessage::GuardarExpediente => {
                if self.expedientes_view.expediente_editando_id.is_some() {
                    // Editar expediente existente
                    if let Some(exp) = self.expedientes_view.obtener_expediente_editado() {
                        let _ = self.storage.actualizar_expediente(exp);
                    }
                } else {
                    // Crear nuevo expediente
                    if let Some(exp) = self.expedientes_view.obtener_expediente_nuevo() {
                        let _ = self.storage.agregar_expediente(exp);
                    }
                }
                self.expedientes_view.cancelar_edicion();
                self.expedientes_view
                    .actualizar_expedientes(self.storage.obtener_expedientes().clone());
            }
            ExpedienteMessage::CancelarEdicion => {
                self.expedientes_view.cancelar_edicion();
            }
        }
    }

    fn manejar_mensaje_ficha(&mut self, mensaje: FichaMessage) {
        if let Vista::Fichas(expediente_id) = &self.vista_actual {
            let expediente_id = *expediente_id;

            match mensaje {
                FichaMessage::NuevaFicha => {
                    self.fichas_view.iniciar_nueva_ficha();
                }
                FichaMessage::EditarFicha(id) => {
                    self.fichas_view.iniciar_edicion(id);
                }
                FichaMessage::EliminarFicha(id) => {
                    if let Some(exp) = self.storage.obtener_expediente_mut(expediente_id) {
                        exp.eliminar_ficha(id);
                        let _ = self.storage.guardar();
                        if let Some(exp_updated) = self.storage.obtener_expediente(expediente_id) {
                            self.fichas_view.actualizar_fichas(
                                exp_updated.fichas.clone(),
                                exp_updated.nombre.clone(),
                            );
                        }
                    }
                }
                FichaMessage::TituloChanged(valor) => {
                    self.fichas_view.titulo_input = valor;
                }
                FichaMessage::DescripcionChanged(valor) => {
                    self.fichas_view.descripcion_input = valor;
                }
                FichaMessage::EstadoChanged(estado) => {
                    self.fichas_view.estado_input = estado.clone();
                    self.fichas_view.nuevo_estado_input = estado;
                }
                FichaMessage::VerHistorial(ficha_id) => {
                    self.fichas_view.ficha_historial_id = Some(ficha_id);
                    self.fichas_view.nuevo_estado_input.clear();
                }
                FichaMessage::CerrarHistorial => {
                    self.fichas_view.ficha_historial_id = None;
                    self.fichas_view.nuevo_estado_input.clear();
                }
                FichaMessage::AgregarEstado(ficha_id) => {
                    if !self.fichas_view.nuevo_estado_input.is_empty() {
                        if let Some(exp) = self.storage.obtener_expediente_mut(expediente_id) {
                            if let Some(ficha) = exp.fichas.iter_mut().find(|f| f.id == ficha_id) {
                                ficha.agregar_estado(self.fichas_view.nuevo_estado_input.clone());
                                let _ = self.storage.guardar();
                                self.fichas_view.nuevo_estado_input.clear();
                            }
                        }
                        if let Some(exp_updated) = self.storage.obtener_expediente(expediente_id) {
                            self.fichas_view.actualizar_fichas(
                                exp_updated.fichas.clone(),
                                exp_updated.nombre.clone(),
                            );
                        }
                    }
                }
                FichaMessage::GuardarFicha => {
                    if let Some(exp) = self.storage.obtener_expediente_mut(expediente_id) {
                        if let Some(ficha_id) = self.fichas_view.ficha_editando_id {
                            // Editar ficha existente
                            if let Some(ficha) = self.fichas_view.obtener_ficha_editada() {
                                exp.actualizar_ficha(ficha_id, ficha);
                            }
                        } else {
                            // Crear nueva ficha
                            if let Some(ficha) = self.fichas_view.obtener_ficha_nueva() {
                                exp.agregar_ficha(ficha);
                            }
                        }
                    }
                    let _ = self.storage.guardar();
                    self.fichas_view.cancelar_edicion();
                    if let Some(exp_updated) = self.storage.obtener_expediente(expediente_id) {
                        self.fichas_view.actualizar_fichas(
                            exp_updated.fichas.clone(),
                            exp_updated.nombre.clone(),
                        );
                    }
                }
                FichaMessage::CancelarEdicion => {
                    self.fichas_view.cancelar_edicion();
                }
                FichaMessage::Volver => {
                    self.vista_actual = Vista::Expedientes;
                }
            }
        }
    }
}
