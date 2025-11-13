use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hoja {
    pub contenido: String,
    pub fecha: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ficha {
    pub id: Uuid,
    pub titulo: String,
    pub descripcion: String,
    pub hojas: Vec<Hoja>,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_modificacion: DateTime<Utc>,
}

impl Ficha {
    pub fn new(titulo: String, descripcion: String, hoja_inicial: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            titulo,
            descripcion,
            hojas: vec![Hoja {
                contenido: hoja_inicial,
                fecha: now,
            }],
            fecha_creacion: now,
            fecha_modificacion: now,
        }
    }

    pub fn actualizar(&mut self, titulo: String, descripcion: String) {
        self.titulo = titulo;
        self.descripcion = descripcion;
        self.fecha_modificacion = Utc::now();
    }

    pub fn agregar_hoja(&mut self, contenido: String) {
        let nueva_hoja = Hoja {
            contenido,
            fecha: Utc::now(),
        };
        // Insertar al inicio para que la última esté siempre primero
        self.hojas.insert(0, nueva_hoja);
        self.fecha_modificacion = Utc::now();
    }

    pub fn hoja_actual(&self) -> Option<&Hoja> {
        self.hojas.first()
    }

    pub fn obtener_hojas(&self) -> &Vec<Hoja> {
        &self.hojas
    }
}
