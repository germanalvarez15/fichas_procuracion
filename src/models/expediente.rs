use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::Ficha;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expediente {
    pub id: Uuid,
    pub numero: String,
    pub nombre: String,
    pub descripcion: String,
    pub fichas: Vec<Ficha>,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_modificacion: DateTime<Utc>,
}

impl Expediente {
    pub fn new(numero: String, nombre: String, descripcion: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            numero,
            nombre,
            descripcion,
            fichas: Vec::new(),
            fecha_creacion: now,
            fecha_modificacion: now,
        }
    }

    pub fn agregar_ficha(&mut self, ficha: Ficha) {
        self.fichas.push(ficha);
        self.fecha_modificacion = Utc::now();
    }

    pub fn eliminar_ficha(&mut self, ficha_id: Uuid) -> bool {
        if let Some(pos) = self.fichas.iter().position(|f| f.id == ficha_id) {
            self.fichas.remove(pos);
            self.fecha_modificacion = Utc::now();
            true
        } else {
            false
        }
    }

    pub fn actualizar_ficha(&mut self, ficha_id: Uuid, ficha: Ficha) -> bool {
        if let Some(f) = self.fichas.iter_mut().find(|f| f.id == ficha_id) {
            *f = ficha;
            self.fecha_modificacion = Utc::now();
            true
        } else {
            false
        }
    }

    pub fn obtener_ficha(&self, ficha_id: Uuid) -> Option<&Ficha> {
        self.fichas.iter().find(|f| f.id == ficha_id)
    }
}
