use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EstadoFicha {
    Pendiente,
    EnProgreso,
    Completada,
    Cancelada,
}

impl fmt::Display for EstadoFicha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl EstadoFicha {
    pub fn as_str(&self) -> &str {
        match self {
            EstadoFicha::Pendiente => "Pendiente",
            EstadoFicha::EnProgreso => "En Progreso",
            EstadoFicha::Completada => "Completada",
            EstadoFicha::Cancelada => "Cancelada",
        }
    }

    pub fn all() -> Vec<EstadoFicha> {
        vec![
            EstadoFicha::Pendiente,
            EstadoFicha::EnProgreso,
            EstadoFicha::Completada,
            EstadoFicha::Cancelada,
        ]
    }
}

impl Default for EstadoFicha {
    fn default() -> Self {
        EstadoFicha::Pendiente
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ficha {
    pub id: Uuid,
    pub titulo: String,
    pub descripcion: String,
    pub estado: EstadoFicha,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_modificacion: DateTime<Utc>,
}

impl Ficha {
    pub fn new(titulo: String, descripcion: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            titulo,
            descripcion,
            estado: EstadoFicha::default(),
            fecha_creacion: now,
            fecha_modificacion: now,
        }
    }

    pub fn actualizar(&mut self, titulo: String, descripcion: String, estado: EstadoFicha) {
        self.titulo = titulo;
        self.descripcion = descripcion;
        self.estado = estado;
        self.fecha_modificacion = Utc::now();
    }
}
