//! Dependency resolution for akmod.

use std::collections::{HashMap, HashSet};

/// Dependency node
#[derive(Debug, Clone)]
pub struct DependencyNode {
    /// Node ID
    pub id: String,
    /// Node name
    pub name: String,
    /// Node dependencies
    pub dependencies: Vec<String>,
    /// Node dependents
    pub dependents: Vec<String>,
}

impl DependencyNode {
    /// Create new dependency node
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            dependencies: vec![],
            dependents: vec![],
        }
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, dependency_id: String) {
        if !self.dependencies.contains(&dependency_id) {
            self.dependencies.push(dependency_id);
        }
    }
    
    /// Remove dependency
    pub fn remove_dependency(&mut self, dependency_id: &str) -> bool {
        if let Some(pos) = self.dependencies.iter().position(|d| d == dependency_id) {
            self.dependencies.remove(pos);
            return true;
        }
        false
    }
    
    /// Add dependent
    pub fn add_dependent(&mut self, dependent_id: String) {
        if !self.dependents.contains(&dependent_id) {
            self.dependents.push(dependent_id);
        }
    }
    
    /// Remove dependent
    pub fn remove_dependent(&mut self, dependent_id: &str) -> bool {
        if let Some(pos) = self.dependents.iter().position(|d| d == dependent_id) {
            self.dependents.remove(pos);
            return true;
        }
        false
    }
}

/// Dependency graph
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Graph nodes
    pub nodes: HashMap<String, DependencyNode>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}

impl DependencyGraph {
    /// Create new dependency graph
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add node
    pub fn add_node(&mut self, node: DependencyNode) -> String {
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        id
    }
    
    /// Remove node
    pub fn remove_node(&mut self, node_id: &str) -> bool {
        if self.nodes.remove(node_id).is_some() {
            // Remove references from other nodes
            for node in self.nodes.values_mut() {
                node.dependencies.retain(|d| d != node_id);
                node.dependents.retain(|d| d != node_id);
            }
            return true;
        }
        false
    }
    
    /// Get node
    pub fn get_node(&self, node_id: &str) -> Option<&DependencyNode> {
        self.nodes.get(node_id)
    }
    
    /// Get node mutably
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut DependencyNode> {
        self.nodes.get_mut(node_id)
    }
    
    /// Add dependency edge
    pub fn add_dependency_edge(&mut self, from: &str, to: &str) -> bool {
        if !self.nodes.contains_key(from) || !self.nodes.contains_key(to) {
            return false;
        }
        
        // Update from node
        if let Some(from_node) = self.nodes.get_mut(from) {
            from_node.add_dependency(to.to_string());
        }
        
        // Update to node
        if let Some(to_node) = self.nodes.get_mut(to) {
            to_node.add_dependent(from.to_string());
        }
        
        true
    }
    
    /// Remove dependency edge
    pub fn remove_dependency_edge(&mut self, from: &str, to: &str) -> bool {
        if !self.nodes.contains_key(from) || !self.nodes.contains_key(to) {
            return false;
        }
        
        // Update from node
        if let Some(from_node) = self.nodes.get_mut(from) {
            from_node.remove_dependency(to);
        }
        
        // Update to node
        if let Some(to_node) = self.nodes.get_mut(to) {
            to_node.remove_dependent(from);
        }
        
        true
    }
    
    /// Get dependencies of node
    pub fn get_dependencies(&self, node_id: &str) -> Vec<String> {
        self.nodes.get(node_id)
            .map(|n| n.dependencies.clone())
            .unwrap_or_default()
    }
    
    /// Get dependents of node
    pub fn get_dependents(&self, node_id: &str) -> Vec<String> {
        self.nodes.get(node_id)
            .map(|n| n.dependents.clone())
            .unwrap_or_default()
    }
    
    /// Get all nodes
    pub fn get_all_nodes(&self) -> Vec<&DependencyNode> {
        self.nodes.values().collect()
    }
    
    /// Get node count
    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Get edge count
    pub fn get_edge_count(&self) -> usize {
        self.nodes.values()
            .map(|n| n.dependencies.len())
            .sum()
    }
    
    /// Check for cycles
    pub fn has_cycles(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        
        for node_id in self.nodes.keys() {
            if self.has_cycle_dfs(node_id, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }
    
    /// DFS for cycle detection
    fn has_cycle_dfs(&self, node_id: &str, visited: &mut HashSet<String>, rec_stack: &mut HashSet<String>) -> bool {
        if rec_stack.contains(node_id) {
            return true;
        }
        
        if visited.contains(node_id) {
            return false;
        }
        
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        
        if let Some(node) = self.nodes.get(node_id) {
            for dep in &node.dependencies {
                if self.has_cycle_dfs(dep, visited, rec_stack) {
                    return true;
                }
            }
        }
        
        rec_stack.remove(node_id);
        false
    }
    
    /// Get topological order
    pub fn get_topological_order(&self) -> Vec<String> {
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut queue = Vec::new();
        let mut result = vec![];
        
        // Initialize in-degree
        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }
        
        // Calculate in-degree
        for node in self.nodes.values() {
            for dep in &node.dependencies {
                if let Some(degree) = in_degree.get_mut(dep) {
                    *degree += 1;
                }
            }
        }
        
        // Add nodes with in-degree 0 to queue
        for (node_id, degree) in &in_degree {
            if *degree == 0 {
                queue.push(node_id.clone());
            }
        }
        
        // Process queue
        while let Some(node_id) = queue.pop() {
            result.push(node_id.clone());
            
            // Remove node and update in-degree of neighbors
            if let Some(node) = self.nodes.get(&node_id) {
                for dependent in &node.dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(dependent.clone());
                        }
                    }
                }
            }
        }
        
        result
    }
}

/// Dependency resolver
#[derive(Debug, Clone)]
pub struct DependencyResolver {
    /// Dependency graph
    pub graph: DependencyGraph,
    /// Resolved dependencies cache
    resolved_cache: HashMap<String, Vec<String>>,
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self {
            graph: DependencyGraph::new(),
            resolved_cache: HashMap::new(),
        }
    }
}

impl DependencyResolver {
    /// Create new dependency resolver
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add dependency
    pub fn add_dependency(&mut self, module_name: String, dependency_name: String) -> bool {
        // Create nodes if they don't exist
        if !self.graph.nodes.contains_key(&module_name) {
            let node = DependencyNode::new(module_name.clone(), module_name.clone());
            self.graph.add_node(node);
        }
        if !self.graph.nodes.contains_key(&dependency_name) {
            let node = DependencyNode::new(dependency_name.clone(), dependency_name.clone());
            self.graph.add_node(node);
        }
        
        // Add dependency edge
        self.graph.add_dependency_edge(&module_name, &dependency_name)
    }
    
    /// Remove dependency
    pub fn remove_dependency(&mut self, module_name: &str, dependency_name: &str) -> bool {
        self.graph.remove_dependency_edge(module_name, dependency_name)
    }
    
    /// Resolve dependencies for module
    pub fn resolve_dependencies(&mut self, module_name: String) -> Vec<String> {
        // Check cache first
        if let Some(cached) = self.resolved_cache.get(&module_name) {
            return cached.clone();
        }
        
        // Get topological order
        let order = self.graph.get_topological_order();
        
        // Find all dependencies of module
        let mut dependencies = vec![module_name.clone()];
        let mut visited = HashSet::new();
        
        self.collect_dependencies(&module_name, &mut dependencies, &mut visited);
        
        // Sort by topological order
        dependencies.sort_by_key(|dep| order.iter().position(|n| n == dep).unwrap_or(usize::MAX));
        
        // Cache result
        self.resolved_cache.insert(module_name, dependencies.clone());
        dependencies
    }
    
    /// Collect dependencies recursively
    fn collect_dependencies(&self, module_name: &str, dependencies: &mut Vec<String>, visited: &mut HashSet<String>) {
        if visited.contains(module_name) {
            return;
        }
        
        visited.insert(module_name.to_string());
        
        if let Some(node) = self.graph.get_node(module_name) {
            for dep in &node.dependencies {
                if !dependencies.contains(dep) {
                    dependencies.push(dep.clone());
                }
                self.collect_dependencies(dep, dependencies, visited);
            }
        }
    }
    
    /// Get all dependencies
    pub fn get_dependencies(&self) -> Vec<String> {
        self.graph.get_all_nodes()
            .iter()
            .map(|n| n.name.clone())
            .collect()
    }
    
    /// Check if dependency exists
    pub fn has_dependency(&self, module_name: &str, dependency_name: &str) -> bool {
        if let Some(node) = self.graph.get_node(module_name) {
            node.dependencies.contains(&dependency_name.to_string())
        } else {
            false
        }
    }
    
    /// Check for circular dependencies
    pub fn has_circular_dependencies(&self) -> bool {
        self.graph.has_cycles()
    }
    
    /// Get dependency graph
    pub fn get_graph(&self) -> &DependencyGraph {
        &self.graph
    }
    
    /// Get dependency graph mutably
    pub fn get_graph_mut(&mut self) -> &mut DependencyGraph {
        &mut self.graph
    }
    
    /// Clear resolver
    pub fn clear(&mut self) {
        self.graph = DependencyGraph::new();
        self.resolved_cache.clear();
    }
    
    /// Get resolver statistics
    pub fn get_statistics(&self) -> ResolverStatistics {
        ResolverStatistics {
            total_nodes: self.graph.get_node_count(),
            total_edges: self.graph.get_edge_count(),
            has_cycles: self.graph.has_cycles(),
            cached_resolutions: self.resolved_cache.len(),
        }
    }
}

/// Resolver statistics
#[derive(Debug, Clone)]
pub struct ResolverStatistics {
    /// Total nodes
    pub total_nodes: usize,
    /// Total edges
    pub total_edges: usize,
    /// Has cycles
    pub has_cycles: bool,
    /// Cached resolutions
    pub cached_resolutions: usize,
}
