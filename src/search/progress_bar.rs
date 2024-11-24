use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Estructura para manejar el progreso de la búsqueda
#[derive(Debug, Clone)]
pub struct ProgressTracker {
    total: Arc<AtomicU64>,
    current: Arc<AtomicU64>,
}

impl ProgressTracker {
    /// Crea un nuevo rastreador de progreso
    pub fn new() -> Self {
        Self {
            total: Arc::new(AtomicU64::new(0)),
            current: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Establece el total de elementos a procesar
    pub fn set_total(&self, total: f32) {
        let total_u64 = total as u64;
        self.total.store(total_u64, Ordering::SeqCst);
        println!("Total establecido: {} (verificación: {})", 
            total_u64, 
            self.total.load(Ordering::SeqCst)
        );
    }

    /// Incrementa el contador actual
    pub fn increment(&self) {
        let nuevo_valor = self.current.fetch_add(1, Ordering::SeqCst) + 1;
        println!("Progreso incrementado: {}", nuevo_valor);
    }

    /// Obtiene el progreso actual como un valor entre 0.0 y 1.0
    pub fn get_progress(&self) -> f32 {
        let total = self.total.load(Ordering::SeqCst) as f32;
        let current = self.current.load(Ordering::SeqCst) as f32;
        
        let progress = if total > 0.0 {
            (current / total).min(1.0)
        } else {
            0.0
        };
        
        println!("Progreso actual: {:.2}% ({}/{})", 
            progress * 100.0, 
            current as u64, 
            total as u64
        );
        progress
    }

    /// Resetea el progreso
    pub fn reset(&self) {
        self.total.store(0, Ordering::SeqCst);
        self.current.store(0, Ordering::SeqCst);
        println!("Progreso reseteado");
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self::new()
    }
}