use crate::models::Ficha;
use crate::ui::styles;
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column, Space};
use iced::{Border, Color, Element, Length};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum FichaMessage {
    NuevaFicha,
    EditarFicha(Uuid),
    EliminarFicha(Uuid),
    VerHistorial(Uuid),
    TituloChanged(String),
    DescripcionChanged(String),
    HojaChanged(String),
    AgregarHoja(Uuid),
    GuardarFicha,
    CancelarEdicion,
    CerrarHistorial,
}

#[derive(Debug, Clone)]
pub struct FichasView {
    pub fichas: Vec<Ficha>,
    pub editando: bool,
    pub titulo_input: String,
    pub descripcion_input: String,
    pub hoja_input: String,
    pub nueva_hoja_input: String,
    pub ficha_editando_id: Option<Uuid>,
    pub ficha_historial_id: Option<Uuid>,
}

impl FichasView {
    pub fn new() -> Self {
        Self {
            fichas: Vec::new(),
            editando: false,
            titulo_input: String::new(),
            descripcion_input: String::new(),
            hoja_input: String::new(),
            nueva_hoja_input: String::new(),
            ficha_editando_id: None,
            ficha_historial_id: None,
        }
    }

    pub fn actualizar_fichas(&mut self, fichas: Vec<Ficha>) {
        self.fichas = fichas;
    }

    pub fn iniciar_nueva_ficha(&mut self) {
        self.editando = true;
        self.titulo_input.clear();
        self.descripcion_input.clear();
        self.hoja_input.clear();
        self.ficha_editando_id = None;
    }

    pub fn iniciar_edicion(&mut self, ficha_id: Uuid) {
        if let Some(ficha) = self.fichas.iter().find(|f| f.id == ficha_id) {
            self.editando = true;
            self.titulo_input = ficha.titulo.clone();
            self.descripcion_input = ficha.descripcion.clone();
            self.hoja_input = ficha
                .hoja_actual()
                .map(|h| h.contenido.clone())
                .unwrap_or_default();
            self.ficha_editando_id = Some(ficha_id);
        }
    }

    pub fn cancelar_edicion(&mut self) {
        self.editando = false;
        self.titulo_input.clear();
        self.descripcion_input.clear();
        self.hoja_input.clear();
        self.ficha_editando_id = None;
    }

    pub fn obtener_ficha_nueva(&self) -> Option<Ficha> {
        if !self.titulo_input.is_empty() && !self.hoja_input.is_empty() {
            Some(Ficha::new(
                self.titulo_input.clone(),
                self.descripcion_input.clone(),
                self.hoja_input.clone(),
            ))
        } else {
            None
        }
    }

    pub fn obtener_ficha_editada(&self) -> Option<Ficha> {
        if let Some(id) = self.ficha_editando_id {
            if let Some(mut ficha) = self.fichas.iter().find(|f| f.id == id).cloned() {
                ficha.actualizar(self.titulo_input.clone(), self.descripcion_input.clone());
                return Some(ficha);
            }
        }
        None
    }

    pub fn view(&self) -> Element<FichaMessage> {
        // Si estamos viendo el historial de una ficha
        if let Some(ficha_id) = self.ficha_historial_id {
            return self.vista_historial(ficha_id);
        }

        let titulo = text("Fichas").size(24);

        let boton_nueva = button(text("+ Nueva Ficha"))
            .on_press(FichaMessage::NuevaFicha)
            .padding(10)
            .style(styles::primary_button);

        let header = row![
            titulo,
            Space::with_width(Length::Fill),
            boton_nueva
        ]
        .spacing(20)
        .padding(10);

        let contenido = if self.editando {
            self.vista_formulario()
        } else {
            self.vista_lista()
        };

        column![header, contenido].spacing(10).padding(20).into()
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

        let hoja_picker = column![
            text("Hoja inicial:"),
            text_input("Contenido de la hoja inicial", &self.hoja_input)
                .on_input(FichaMessage::HojaChanged)
                .padding(8),
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

        column![
            titulo,
            titulo_input,
            descripcion_input,
            hoja_picker,
            botones
        ]
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

        // Agrupar las cards en filas de 3
        let cards_per_row = 3;
        let mut rows_container = Column::new().spacing(15).padding(10);
        
        for chunk in self.fichas.chunks(cards_per_row) {
            let mut row_elements = iced::widget::Row::new().spacing(15);
            
            for ficha in chunk {
                let hoja_actual = ficha
                    .hoja_actual()
                    .map(|h| h.contenido.as_str())
                    .unwrap_or("Sin hoja");

                let fecha_hoja_actual = ficha
                    .hoja_actual()
                    .map(|h| h.fecha.format("%d/%m/%Y %H:%M").to_string())
                    .unwrap_or("Sin fecha".into());

                let card = container(
                    column![
                        text(&ficha.titulo).size(18),
                        text(&ficha.descripcion).size(14),
                        row![
                            text("Ultima Hoja: ").size(12),
                            text(hoja_actual).size(12).font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            }),
                            text(", creada el: ").size(12),
                            text(fecha_hoja_actual).size(10).font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..Default::default()
                            }),
                        ],
                        text(format!(
                            "Creada: {}",
                            ficha.fecha_creacion.format("%d/%m/%Y %H:%M")
                        ))
                        .size(11),
                        Space::with_height(Length::Fill),
                        row![
                            button(text("Ver Hojas").center())
                                .on_press(FichaMessage::VerHistorial(ficha.id))
                                .padding(8)
                                .width(Length::Fixed(110.0))
                                .style(styles::primary_button),
                            button(text("Editar").center())
                                .on_press(FichaMessage::EditarFicha(ficha.id))
                                .padding(8)
                                .width(Length::Fixed(85.0))
                                .style(styles::secondary_button),
                            button(text("Eliminar").center())
                                .on_press(FichaMessage::EliminarFicha(ficha.id))
                                .width(Length::Fixed(90.0))
                                .style(styles::cancel_button)
                                .padding(8),
                        ]
                        .spacing(8),
                    ]
                    .spacing(8),
                )
                .padding(15)
                .width(Length::Fixed(320.0))
                .height(Length::Fixed(240.0))
                .style(styles::card_container);

                row_elements = row_elements.push(card);
            }
            
            rows_container = rows_container.push(row_elements);
        }

        scrollable(rows_container).into()
    }

    fn vista_historial(&self, ficha_id: Uuid) -> Element<FichaMessage> {
        let ficha = self.fichas.iter().find(|f| f.id == ficha_id);

        if let Some(ficha) = ficha {
            let titulo = text(format!("Hojas de la Ficha - {}", ficha.titulo)).size(24);

            let boton_volver = button(text("← Volver"))
                .on_press(FichaMessage::CerrarHistorial)
                .padding(10)
                .style(styles::secondary_button);

            let header = row![boton_volver, Space::with_width(Length::Fill), titulo]
                .spacing(20)
                .padding(10);

            // Formulario para agregar nueva hoja
            let nueva_hoja_form = container(
                column![
                    text("Agregar nueva hoja:").size(16),
                    row![
                        text_input("Ingrese el contenido de la nueva hoja...", &self.nueva_hoja_input)
                            .on_input(FichaMessage::HojaChanged)
                            .padding(8)
                            .width(Length::Fill),
                        button(text("Agregar"))
                            .on_press(FichaMessage::AgregarHoja(ficha_id))
                            .padding(8)
                            .style(styles::primary_button),
                    ]
                    .spacing(10),
                ]
                .spacing(10),
            )
            .padding(15)
            .width(Length::Fill)
            .style(styles::card_container);

            // Lista de hojas
            let hojas_list: Element<_> = ficha
                .obtener_hojas()
                .iter()
                .fold(Column::new().spacing(8), |column, hoja| {
                    let hoja_card = container(
                        column![
                            text(&hoja.contenido).size(16),
                            text(format!(
                                "Fecha: {}",
                                hoja.fecha.format("%d/%m/%Y %H:%M:%S")
                            ))
                            .size(12),
                        ]
                        .spacing(5),
                    )
                    .padding(12)
                    .width(Length::Fill)
                    .style(|_theme| container::Style {
                        background: Some(Color::from_rgb(0.95, 0.97, 0.99).into()),
                        border: Border {
                            color: Color::from_rgb(0.7, 0.8, 0.9),
                            width: 1.0,
                            radius: 5.0.into(),
                        },
                        ..Default::default()
                    });

                    column.push(hoja_card)
                })
                .into();

            let hojas_container = container(
                column![
                    text("Hojas (más reciente primero):").size(16),
                    hojas_list,
                ]
                .spacing(10),
            )
            .padding(15)
            .width(Length::Fill);

            column![header, nueva_hoja_form, hojas_container]
                .spacing(15)
                .padding(20)
                .into()
        } else {
            container(text("Ficha no encontrada")).padding(20).into()
        }
    }
}
