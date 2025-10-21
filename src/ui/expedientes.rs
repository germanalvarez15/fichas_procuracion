use iced::widget::{button, column, container, row, scrollable, text, text_input, Column, Space};
use iced::{Element, Length, Color, Border};
use crate::models::Expediente;
use crate::ui::styles;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ExpedienteMessage {
    NuevoExpediente,
    EditarExpediente(Uuid),
    EliminarExpediente(Uuid),
    SeleccionarExpediente(Uuid),
    NumeroChanged(String),
    NombreChanged(String),
    DescripcionChanged(String),
    GuardarExpediente,
    CancelarEdicion,
}

#[derive(Debug, Clone)]
pub struct ExpedientesView {
    pub expedientes: Vec<Expediente>,
    pub expediente_seleccionado: Option<Uuid>,
    pub editando: bool,
    pub numero_input: String,
    pub nombre_input: String,
    pub descripcion_input: String,
    pub expediente_editando_id: Option<Uuid>,
}

impl ExpedientesView {
    pub fn new() -> Self {
        Self {
            expedientes: Vec::new(),
            expediente_seleccionado: None,
            editando: false,
            numero_input: String::new(),
            nombre_input: String::new(),
            descripcion_input: String::new(),
            expediente_editando_id: None,
        }
    }

    pub fn actualizar_expedientes(&mut self, expedientes: Vec<Expediente>) {
        self.expedientes = expedientes;
    }

    pub fn iniciar_nuevo_expediente(&mut self) {
        self.editando = true;
        self.numero_input.clear();
        self.nombre_input.clear();
        self.descripcion_input.clear();
        self.expediente_editando_id = None;
    }

    pub fn iniciar_edicion(&mut self, expediente_id: Uuid) {
        if let Some(exp) = self.expedientes.iter().find(|e| e.id == expediente_id) {
            self.editando = true;
            self.numero_input = exp.numero.clone();
            self.nombre_input = exp.nombre.clone();
            self.descripcion_input = exp.descripcion.clone();
            self.expediente_editando_id = Some(expediente_id);
        }
    }

    pub fn cancelar_edicion(&mut self) {
        self.editando = false;
        self.numero_input.clear();
        self.nombre_input.clear();
        self.descripcion_input.clear();
        self.expediente_editando_id = None;
    }

    pub fn obtener_expediente_nuevo(&self) -> Option<Expediente> {
        if !self.numero_input.is_empty() && !self.nombre_input.is_empty() {
            Some(Expediente::new(
                self.numero_input.clone(),
                self.nombre_input.clone(),
                self.descripcion_input.clone(),
            ))
        } else {
            None
        }
    }

    pub fn obtener_expediente_editado(&self) -> Option<Expediente> {
        if let Some(id) = self.expediente_editando_id {
            if let Some(mut exp) = self.expedientes.iter().find(|e| e.id == id).cloned() {
                exp.numero = self.numero_input.clone();
                exp.nombre = self.nombre_input.clone();
                exp.descripcion = self.descripcion_input.clone();
                return Some(exp);
            }
        }
        None
    }

    pub fn view(&self) -> Element<ExpedienteMessage> {
        let titulo = text("Expedientes").size(24);

        let boton_nuevo = button(text("+ Nuevo Expediente"))
            .on_press(ExpedienteMessage::NuevoExpediente)
            .padding(10);

        let header = row![
            titulo,
            Space::with_width(Length::Fill), // Espacio flexible que empuja el botón a la derecha
            boton_nuevo
        ]
        .padding(10);

        let contenido = if self.editando {
            self.vista_formulario()
        } else {
            self.vista_lista()
        };

        column![header, contenido]
            .spacing(10)
            .padding(20)
            .into()
    }

    fn vista_formulario(&self) -> Element<ExpedienteMessage> {
        let titulo = if self.expediente_editando_id.is_some() {
            text("Editar Expediente").size(20)
        } else {
            text("Nuevo Expediente").size(20)
        };

        let numero = column![
            text("Número:"),
            text_input("Ej: EXP-2024-001", &self.numero_input)
                .on_input(ExpedienteMessage::NumeroChanged)
                .padding(8),
        ]
        .spacing(5);

        let nombre = column![
            text("Nombre:"),
            text_input("Nombre del expediente", &self.nombre_input)
                .on_input(ExpedienteMessage::NombreChanged)
                .padding(8),
        ]
        .spacing(5);

        let descripcion = column![
            text("Descripción:"),
            text_input("Descripción del expediente", &self.descripcion_input)
                .on_input(ExpedienteMessage::DescripcionChanged)
                .padding(8),
        ]
        .spacing(5);

        let botones = row![
            button(text("Guardar"))
                .on_press(ExpedienteMessage::GuardarExpediente)
                .padding(10),
            button(text("Cancelar"))
                .on_press(ExpedienteMessage::CancelarEdicion)
                .padding(10),
        ]
        .spacing(10);

        column![titulo, numero, nombre, descripcion, botones]
            .spacing(15)
            .padding(20)
            .into()
    }

    fn vista_lista(&self) -> Element<ExpedienteMessage> {
        if self.expedientes.is_empty() {
            return container(text("No hay expedientes. Crea uno nuevo."))
                .padding(20)
                .into();
        }

        let lista: Element<_> = self
            .expedientes
            .iter()
            .fold(Column::new().spacing(10), |column, expediente| {
                let card = container(
                    column![
                        row![
                            text(&expediente.numero).size(16),
                            text(&expediente.nombre).size(18),
                        ]
                        .spacing(10),
                        text(&expediente.descripcion).size(14),
                        text(format!("Fichas: {}", expediente.fichas.len())).size(12),
                        row![
                            button(text("Ver").center())
                                .on_press(ExpedienteMessage::SeleccionarExpediente(expediente.id))
                                .padding(8)
                                .width(Length::Fixed(100.0))
                                .style(styles::primary_button),
                            button(text("Editar").center())
                                .on_press(ExpedienteMessage::EditarExpediente(expediente.id))
                                .padding(8)
                                .width(Length::Fixed(100.0))
                                .style(styles::secondary_button),
                            button(text("Eliminar").center())
                                .on_press(ExpedienteMessage::EliminarExpediente(expediente.id))
                                .padding(8)
                                .width(Length::Fixed(100.0))
                                .style(styles::cancel_button),
                        ]
                        .spacing(10),
                    ]
                    .spacing(8),
                )
                .padding(15)
                .width(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Color::from_rgb(0.95, 0.95, 0.97).into()),
                    border: Border {
                        color: Color::from_rgb(0.8, 0.8, 0.8),
                        width: 1.0,
                        radius: 5.0.into(),
                    },
                    ..Default::default()
                });

                column.push(card)
            })
            .into();

        scrollable(lista).into()
    }
}
