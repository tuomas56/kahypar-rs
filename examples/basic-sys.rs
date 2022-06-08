use kahypar::sys::*;
use std::ffi::CString;

fn main() {
    unsafe {
        let context = kahypar_context_new();
        let ini_file_name = CString::new("examples/config.ini").unwrap();
        kahypar_configure_context_from_file(context, ini_file_name.as_ptr());

        const NUM_VERTICES: kahypar_hyperedge_id_t = 7;
        const NUM_HYPEREDGES: kahypar_hyperedge_id_t = 4;

        let mut hyperedge_weights: [kahypar_hyperedge_weight_t; 4] = Default::default();
        hyperedge_weights[0] = 1;  hyperedge_weights[1] = 1000;
        hyperedge_weights[2] = 1;  hyperedge_weights[3] = 1000;

        let mut hyperedge_indices: [u64; 5] = Default::default();
        hyperedge_indices[0] = 0; hyperedge_indices[1] = 2;
        hyperedge_indices[2] = 6; hyperedge_indices[3] = 9;
        hyperedge_indices[4] = 12;

        let mut hyperedges: [kahypar_hyperedge_id_t; 12] = Default::default();
        hyperedges[0] = 0;  hyperedges[1] = 2;
        hyperedges[2] = 0;  hyperedges[3] = 1;
        hyperedges[4] = 3;  hyperedges[5] = 4;
        hyperedges[6] = 3;  hyperedges[7] = 4;
        hyperedges[8] = 6;  hyperedges[9] = 2;
        hyperedges[10] = 5; hyperedges[11] = 6;

        let imbalance: f64 = 0.03;
        let k: kahypar_partition_id_t = 2;

        let mut objective: kahypar_hyperedge_weight_t = 0;
        let mut partition: [kahypar_partition_id_t; NUM_VERTICES as usize] = [-1; NUM_VERTICES as usize];

        kahypar_partition(
            NUM_VERTICES, NUM_HYPEREDGES,
            imbalance, k,
            std::ptr::null(), hyperedge_weights.as_ptr(),
            hyperedge_indices.as_ptr(), hyperedges.as_ptr(),
            &mut objective as *mut _, context, partition.as_mut_ptr()
        );

        println!("{:?}", partition);

        kahypar_context_free(context);
    }
}