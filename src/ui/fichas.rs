use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input, Column, Space};
use iced::{Element, Length, Color, Border};
use crate::models::{Ficha, EstadoFicha};
use uuid::Uuid;
use crate::ui::styles;

#[derive(Debug, Clone)]
pub enum FichaMessage {
    NuevaFicha,
    EditarFicha(Uuid),
    EliminarFicha(Uuid),
    TituloChanged(String),
    DescripcionChanged(String),
    EstadoChanged(EstadoFicha),
    GuardarFicha,
    CancelarEdicion,
    Volver,
}

#[derive(Debug, Clone)]
pub struct FichasView {
    pub fichas: Vec<Ficha>,
    pub editando: bool,
    pub titulo_input: String,
    pub descripcion_input: String,
    pub estado_input: EstadoFicha,
    pub ficha_editando_id: Option<Uuid>,
    pub expediente_nombre: String,
}

impl FichasView {
    pub fn new() -> Self {
        Self {
            fichas: Vec::new(),
            editando: false,
            titulo_input: String::new(),
            descripcion_input: String::new(),
            estado_input: EstadoFicha::default(),
            ficha_editando_id: None,
            expediente_nombre: String::new(),
        }
    }

    pub fn actualizar_fichas(&mut self, fichas: Vec<Ficha>, expediente_nombre: String) {
        self.fichas = fichas;
        self.expediente_nombre = expediente_nombre;
    }

    pub fn iniciar_nueva_ficha(&mut self) {
        self.editando = true;
        self.titulo_input.clear();
        self.descripcion_input.clear();
        self.estado_input = EstadoFicha::default();
        self.ficha_editando_id = None;
    }

    pub fn iniciar_edicion(&mut self, ficha_id: Uuid) {
        if let Some(ficha) = self.fichas.iter().find(|f| f.id == ficha_id) {
            self.editando = true;
            self.titulo_input = ficha.titulo.clone();
            self.descripcion_input = ficha.descripcion.clone();
            self.estado_input = ficha.estado.clone();
            self.ficha_editando_id = Some(ficha_id);
        }
    }

    pub fn cancelar_edicion(&mut self) {
        self.editando = false;
        self.titulo_input.clear();
        self.descripcion_input.clear();
        self.estado_input = EstadoFicha::default();
        self.ficha_editando_id = None;
    }

    pub fn obtener_ficha_nueva(&self) -> Option<Ficha> {
        if !self.titulo_input.is_empty() {
            Some(Ficha::new(
                self.titulo_input.clone(),
                self.descripcion_input.clone(),
            ))
        } else {
            None
        }
    }

    pub fn obtener_ficha_editada(&self) -> Option<Ficha> {
        if let Some(id) = self.ficha_editando_id {
            if let Some(mut ficha) = self.fichas.iter().find(|f| f.id == id).cloned() {
                ficha.actualizar(
                    self.titulo_input.clone(),
                    self.descripcion_input.clone(),
                    self.estado_input.clone(),
                );
                return Some(ficha);
            }
        }
        None
    }

    pub fn view(&self) -> Element<FichaMessage> {
        let titulo = text(format!("Fichas - {}", self.expediente_nombre)).size(24);

        let boton_volver = button(text("← Volver"))
            .on_press(FichaMessage::Volver)
            .padding(10);

        let boton_nueva = button(text("+ Nueva Ficha"))
            .on_press(FichaMessage::NuevaFicha)
            .padding(10);

        let header = row![boton_volver,Space::with_width(Length::Fill), titulo, Space::with_width(Length::Fill),boton_nueva]
            .spacing(20)
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

    fn vista_formulario(&self) -> Element<FichaMessage> {
        let titulo = if self.ficha_editando_id.is_some() {
            text("Editar Ficha").size(20)
        } else {
            text("Nueva Ficha").size(20)
        };

        let titulo_input = column![
            text("Título:"),
            text_input("Título de la ficha", &self.titulo_input)
                .on_input(FichaMessage::TituloChanged)
                .padding(8),
        ]
        .spacing(5);

        let descripcion_input = column![
            text("Descripción:"),
            text_input("Descripción de la ficha", &self.descripcion_input)
                .on_input(FichaMessage::DescripcionChanged)
                .padding(8),
        ]
        .spacing(5);

        let estado_picker = column![
            text("Estado:"),
            pick_list(
                EstadoFicha::all(),
                Some(self.estado_input.clone()),
                FichaMessage::EstadoChanged
            ),
        ]
        .spacing(5);

        let botones = row![
            button(text("Guardar"))
                .on_press(FichaMessage::GuardarFicha)
                .padding(10),
            button(text("Cancelar"))
                .on_press(FichaMessage::CancelarEdicion)
                .padding(10),
        ]
        .spacing(10);

        column![titulo, titulo_input, descripcion_input, estado_picker, botones]
            .spacing(15)
            .padding(20)
            .into()
    }

    fn vista_lista(&self) -> Element<FichaMessage> {
        if self.fichas.is_empty() {
            return container(text("No hay fichas. Crea una nueva."))
                .padding(20)
                .into();
        }

        let lista: Element<_> = self
            .fichas
            .iter()
            .fold(Column::new().spacing(10), |column, ficha| {
                let card = container(
                    column![
                        text(&ficha.titulo).size(18),
                        text(&ficha.descripcion).size(14),
                        text(format!("Estado: {}", ficha.estado.as_str())).size(12),
                        text(format!(
                            "Creada: {}",
                            ficha.fecha_creacion.format("%d/%m/%Y %H:%M")
                        ))
                        .size(11),
                        row![
                            button(text("Editar"))
                                .on_press(FichaMessage::EditarFicha(ficha.id))
                                .padding(8)
                                .style(styles::secondary_button),
                            button(text("Eliminar"))
                                .on_press(FichaMessage::EliminarFicha(ficha.id))
                                .style(styles::cancel_button)
                                .padding(8),
                        ]
                        .spacing(10),
                    ]
                    .spacing(8),
                )
                .padding(15)
                .width(Length::Fill);

                column.push(card)
            })
            .into();

        scrollable(lista).into()
    }
}
