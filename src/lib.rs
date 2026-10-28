// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-apache-exporter - Cloud/ML/Blockchain component
//! Part of SigmaOS multi-repository architecture

#![no_std]

/// Component
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new() -> Self {
        Self { name: "SigmaOS-apache-exporter" }
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creation() {
        let c = super::Component::new();
        assert_eq!(c.name, "SigmaOS-apache-exporter");
    }
}
