use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstadoHistorial {
    pub estado: String,
    pub fecha: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ficha {
    pub id: Uuid,
    pub titulo: String,
    pub descripcion: String,
    pub historial_estados: Vec<EstadoHistorial>,
    pub fecha_creacion: DateTime<Utc>,
    pub fecha_modificacion: DateTime<Utc>,
}

impl Ficha {
    pub fn new(titulo: String, descripcion: String, estado_inicial: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            titulo,
            descripcion,
            historial_estados: vec![EstadoHistorial {
                estado: estado_inicial,
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

    pub fn agregar_estado(&mut self, estado: String) {
        let nuevo_estado = EstadoHistorial {
            estado,
            fecha: Utc::now(),
        };
        // Insertar al inicio para que el último esté siempre primero
        self.historial_estados.insert(0, nuevo_estado);
        self.fecha_modificacion = Utc::now();
    }

    pub fn estado_actual(&self) -> Option<&EstadoHistorial> {
        self.historial_estados.first()
    }

    pub fn obtener_historial(&self) -> &Vec<EstadoHistorial> {
        &self.historial_estados
    }
}
