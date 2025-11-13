use crate::models::Ficha;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonStorage {
    fichas: Vec<Ficha>,
    #[serde(skip)]
    file_path: PathBuf,
}

impl JsonStorage {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            fichas: Vec::new(),
            file_path,
        }
    }

    /// Carga los datos desde el archivo JSON
    pub fn cargar(&mut self) -> io::Result<()> {
        if !self.file_path.exists() {
            // Si el archivo no existe, crear uno vacío
            return self.guardar();
        }

        let contenido = fs::read_to_string(&self.file_path)?;
        let fichas: Vec<Ficha> = serde_json::from_str(&contenido)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.fichas = fichas;
        Ok(())
    }

    /// Guarda los datos en el archivo JSON
    pub fn guardar(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.fichas)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Crear el directorio si no existe
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Obtiene todas las fichas
    pub fn obtener_fichas(&self) -> &Vec<Ficha> {
        &self.fichas
    }

    /// Agrega una nueva ficha
    pub fn agregar_ficha(&mut self, ficha: Ficha) -> io::Result<()> {
        self.fichas.push(ficha);
        self.guardar()
    }

    /// Elimina una ficha por ID
    pub fn eliminar_ficha(&mut self, ficha_id: Uuid) -> io::Result<bool> {
        if let Some(pos) = self.fichas.iter().position(|f| f.id == ficha_id) {
            self.fichas.remove(pos);
            self.guardar()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Actualiza una ficha existente
    pub fn actualizar_ficha(&mut self, ficha: Ficha) -> io::Result<bool> {
        if let Some(f) = self.fichas.iter_mut().find(|f| f.id == ficha.id) {
            *f = ficha;
            self.guardar()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Obtiene una ficha por ID
    pub fn obtener_ficha(&self, ficha_id: Uuid) -> Option<&Ficha> {
        self.fichas.iter().find(|f| f.id == ficha_id)
    }

    /// Obtiene una ficha mutable por ID
    pub fn obtener_ficha_mut(&mut self, ficha_id: Uuid) -> Option<&mut Ficha> {
        self.fichas.iter_mut().find(|f| f.id == ficha_id)
    }
}
