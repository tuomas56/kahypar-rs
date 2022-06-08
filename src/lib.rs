#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ops::Index;
use std::path::Path;
use std::ffi::CString;
use std::borrow::Cow;

pub mod sys;

pub struct Context {
    inner: *mut sys::kahypar_context_s
}

impl Context {
    pub fn from_config<P: AsRef<Path>>(path: P) -> Context {
        let path = path.as_ref().to_str().expect("path must be a valid rust string");
        let path = CString::new(path).expect("path must be a valid C string");
        let inner;
        unsafe {
            inner = sys::kahypar_context_new();
            sys::kahypar_configure_context_from_file(inner, path.as_ptr());
        }

        Context { inner }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            sys::kahypar_context_free(self.inner);
        }
    }
} 

pub struct Hypergraph {
    vertices: usize,
    blocks: sys::kahypar_partition_id_t,
    inner: *mut sys::kahypar_hypergraph_s
}

impl Hypergraph {
    pub fn from_raw<'a>(blocks: usize, vertices: usize, edges_flat: &'a [u32], edge_indices: &'a [u64]) -> HypergraphBuilder<'a> {
        HypergraphBuilder { 
            blocks, vertices, edges: edge_indices.len() - 1,
            edges_flat: edges_flat.into(), edge_indices: edge_indices.into(),
            edge_weights: None, vertex_weights: None, fixed_vertices: None
        }
    }

    pub fn from_edges<'a, U: AsRef<[u32]>> (blocks: usize, vertices: usize, edges: &[U]) -> HypergraphBuilder<'a> {
        let mut edges_flat = Vec::new();
        let mut edge_indices = vec![0];
        
        for edge in edges {
            let edge = edge.as_ref();
            edges_flat.extend_from_slice(edge);
            edge_indices.push(edges_flat.len() as u64);
        }

        HypergraphBuilder { 
            blocks, vertices, edges: edges.len(),
            edges_flat: edges_flat.into(), edge_indices: edge_indices.into(),
            edge_weights: None, vertex_weights: None, fixed_vertices: None
        }
    }

    pub fn from_incidence<'a, U, T>(blocks: usize, vertices: usize, edges: usize, incidence: U) -> HypergraphBuilder<'a>
    where U: Index<(usize, usize), Output=T>, T: Copy + Into<bool> {
        let mut edges_flat = Vec::new();
        let mut edge_indices = vec![0];
        
        for edge in 0..edges {
            for vertex in 0..vertices {
                if incidence[(vertex, edge)].into() {
                    edges_flat.push(vertex as u32);
                }
            }
            edge_indices.push(edges_flat.len() as u64);
        }

        HypergraphBuilder { 
            blocks, vertices, edges,
            edges_flat: edges_flat.into(), edge_indices: edge_indices.into(),
            edge_weights: None, vertex_weights: None, fixed_vertices: None
        }
    }

    pub fn partition(&mut self, context: &mut Context, epsilon: f64) -> (i32, Vec<i32>) {
        let mut objective: sys::kahypar_hyperedge_weight_t = 0;
        let mut partition = vec![-1 as sys::kahypar_partition_id_t; self.vertices];

        unsafe {
            sys::kahypar_partition_hypergraph(
                self.inner, self.blocks, 
                epsilon, &mut objective as *mut _, 
                context.inner, partition.as_mut_ptr()
            );
        }

        (objective as i32, partition)
    }

    pub fn improve_partition(&mut self, context: &mut Context, epsilon: f64, previous: &[i32], iterations: usize) -> (i32, Vec<i32>) {
        let mut objective: sys::kahypar_hyperedge_weight_t = 0;
        let mut partition = vec![-1 as sys::kahypar_partition_id_t; self.vertices];

        unsafe {
            sys::kahypar_improve_hypergraph_partition(
                self.inner, self.blocks, 
                epsilon, &mut objective as *mut _, 
                context.inner, previous.as_ptr(),
                iterations as u64, partition.as_mut_ptr()
            );
        }

        (objective as i32, partition)
    }
}

impl Drop for Hypergraph {
    fn drop(&mut self) {
        unsafe {
            sys::kahypar_hypergraph_free(self.inner);
        }
    }
}

pub struct HypergraphBuilder<'a> {
    blocks: usize,
    vertices: usize,
    edges: usize,
    edges_flat: Cow<'a, [u32]>,
    edge_indices: Cow<'a, [u64]>,
    edge_weights: Option<&'a [i32]>,
    vertex_weights: Option<&'a [i32]>,
    fixed_vertices: Option<&'a [i32]>
}

impl<'a> HypergraphBuilder<'a> {
    pub fn edge_weights(mut self, weights: &'a [i32]) -> Self {
        self.edge_weights = Some(weights);
        self
    }

    pub fn vertex_weights(mut self, weights: &'a [i32]) -> Self {
        self.vertex_weights = Some(weights);
        self
    }

    pub fn fixed_vertices(mut self, partitions: &'a [i32]) -> Self {
        self.fixed_vertices = Some(partitions);
        self
    }

    pub fn build(self) -> Hypergraph {
        let edge_weights = self.edge_weights
            .map(|weights| weights.as_ptr())
            .unwrap_or(std::ptr::null());
        let vertex_weights = self.vertex_weights
            .map(|weights| weights.as_ptr())
            .unwrap_or(std::ptr::null());

        let inner = unsafe {
            sys::kahypar_create_hypergraph(
                self.blocks as sys::kahypar_partition_id_t,
                self.vertices as sys::kahypar_hypernode_id_t,
                self.edges as sys::kahypar_hyperedge_id_t,
                self.edge_indices.as_ptr(),
                self.edges_flat.as_ptr(),
                edge_weights, vertex_weights
            )
        };

        if let Some(partitions) = self.fixed_vertices {
            unsafe {
                sys::kahypar_set_fixed_vertices(inner, partitions.as_ptr());
            }
        }

        Hypergraph { vertices: self.vertices, blocks: self.blocks as sys::kahypar_partition_id_t, inner }
    }
}