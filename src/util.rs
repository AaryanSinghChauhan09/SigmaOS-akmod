//! akmod model — Automatic Kernel Module Building.

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub version: String,
    pub source: String,
    pub kernels: Vec<String>,
    pub built: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub modules: Vec<Module>,
}

impl Default for Model {
    fn default() -> Self { Self { modules: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_module(&mut self, name: &str, version: &str, source: &str) -> usize {
        let id = self.modules.len();
        self.modules.push(Module { name: name.into(), version: version.into(), source: source.into(), kernels: vec![], built: vec![] });
        id
    }

    pub fn build(&mut self, module_id: usize, kernel: &str) -> bool {
        if let Some(m) = self.modules.get_mut(module_id) {
            if !m.kernels.contains(&kernel.to_string()) {
                m.kernels.push(kernel.into());
            }
            if !m.built.contains(&kernel.to_string()) {
                m.built.push(kernel.into());
            }
            true
        } else { false }
    }

    pub fn install(&mut self, module_id: usize, kernel: &str) -> bool {
        self.build(module_id, kernel)
    }

    pub fn remove(&mut self, module_id: usize, kernel: &str) -> bool {
        if let Some(m) = self.modules.get_mut(module_id) {
            m.kernels.retain(|k| k != kernel);
            m.built.retain(|k| k != kernel);
            true
        } else { false }
    }

    pub fn list(&self) -> &[Module] { &self.modules }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_install() {
        let mut a = Model::new();
        let id = a.add_module("nvidia", "550", "/usr/src/nvidia");
        assert!(a.build(id, "6.8.0"));
        assert!(a.install(id, "6.8.0"));
    }
}
