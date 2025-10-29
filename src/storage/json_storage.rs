use crate::models::Expediente;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonStorage {
    expedientes: Vec<Expediente>,
    #[serde(skip)]
    file_path: PathBuf,
}

impl JsonStorage {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            expedientes: Vec::new(),
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
        let expedientes: Vec<Expediente> = serde_json::from_str(&contenido)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        self.expedientes = expedientes;
        Ok(())
    }

    /// Guarda los datos en el archivo JSON
    pub fn guardar(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.expedientes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // Crear el directorio si no existe
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(&self.file_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    /// Obtiene todos los expedientes
    pub fn obtener_expedientes(&self) -> &Vec<Expediente> {
        &self.expedientes
    }

    /// Agrega un nuevo expediente
    pub fn agregar_expediente(&mut self, expediente: Expediente) -> io::Result<()> {
        self.expedientes.push(expediente);
        self.guardar()
    }

    /// Elimina un expediente por ID
    pub fn eliminar_expediente(&mut self, expediente_id: Uuid) -> io::Result<bool> {
        if let Some(pos) = self.expedientes.iter().position(|e| e.id == expediente_id) {
            self.expedientes.remove(pos);
            self.guardar()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Actualiza un expediente existente
    pub fn actualizar_expediente(&mut self, expediente: Expediente) -> io::Result<bool> {
        if let Some(exp) = self.expedientes.iter_mut().find(|e| e.id == expediente.id) {
            *exp = expediente;
            self.guardar()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Obtiene un expediente por ID
    pub fn obtener_expediente(&self, expediente_id: Uuid) -> Option<&Expediente> {
        self.expedientes.iter().find(|e| e.id == expediente_id)
    }

    /// Obtiene un expediente mutable por ID
    pub fn obtener_expediente_mut(&mut self, expediente_id: Uuid) -> Option<&mut Expediente> {
        self.expedientes.iter_mut().find(|e| e.id == expediente_id)
    }
}
