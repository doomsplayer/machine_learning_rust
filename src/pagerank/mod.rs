//! Pagerank -- Pagerank Algorithm in Rust
#![stable]
#![deny(missing_doc)]
extern crate matrixrs;
extern crate libc;

use self::matrixrs::{Matrix,zeros,ToMatrix};
use self::libc::{c_ulong, c_double};
use std::mem::transmute;
use std::ptr;

#[cfg(not(test))]
#[no_mangle] 
pub unsafe fn pagerank_c(
	adjm: *const c_double,
	adjm_size: c_ulong,
	rank: *const c_double,
	max_iter: c_ulong,
	q: c_double,
	eps: c_double,
	ret_matrix: *const c_double
	) {
	//! export to C interface

	let adjm_vec: &[f64] = transmute((adjm, adjm_size * adjm_size));
	let adjmatrix = adjm_vec.iter().map(|x| x.clone()).to_matrix(adjm_size as uint, adjm_size as uint);
	let rank_vec:&[f64] = transmute((rank, adjm_size));
	let rankmatrix = rank_vec.iter().map(|x| x.clone()).to_matrix(adjm_size as uint, 1);
	let ret = pagerank(&adjmatrix, &rankmatrix, max_iter as uint, q, eps);
	// let ret_v: &mut [f64] = transmute((ret_matrix, adjm_size));

	// let buf = alloc::heap::allocate(adjm_size as uint, std::mem::min_align_of::<c_double>());
	// let dst: &mut [c_double] = transmute((buf, adjm_size));

	for (index, i) in ret.iter().enumerate() {
		ptr::write(ret_matrix.offset(index as int) as *mut c_double, i as c_double);
		// dst[index] = i; 
	}
	// buf as *const c_double
}

pub fn pagerank(adjm: &Matrix<f64>, rank: &Matrix<f64>, max_iter: uint, q: f64, eps: f64) -> Matrix<f64> {
	//! adjm is the adjacent matrix, and rank is the initial rank matrix,
	//! and max_iter is the maximum iterater time, and q is conventional 0.85,
	//! and eps is the minimum difference bitween two iterations according to L1 norm.
	let adjm = adjm.clone();
	let mut rank = rank.clone();

	let tmp = (adjm * [1f64, 1., 1., 1.].iter().map(|x| x.clone()).to_matrix(1,4).transpose()).map(|v| 1. / v);

	let mut z: Matrix<f64> = zeros(adjm.nrow, adjm.ncol);
	for (i,v) in tmp.iter().enumerate() {
		z.set(i+1, i+1 ,v)
	}

	let mt = !(z * adjm);

	let mt = Matrix::from_fn(mt.nrow, mt.ncol, |i,j| mt.at(i,j) * q);

	let e = Matrix::from_fn(mt.nrow, mt.ncol, |_,_| (1f64-q) / mt.ncol as f64);
	for _ in range(0,max_iter) {
		let new_rank = (mt + e)* rank;
		if (rank - new_rank).iter().fold(0f64,|acc,b| acc + b.abs()) < eps {
			return new_rank
		} else {
			rank = new_rank
		}
	}
	rank
}

#[cfg(test)]
mod test {
	#[phase(plugin, link)] extern crate matrixrs;
	use self::matrixrs::{ToMatrix,Matrix};
	#[test]
	fn test_1() {
	let adjm: Matrix<f64> = matrix!(
		[0., 1., 0., 1.]
		[1., 0., 1., 0.]
		[0., 1., 0., 0.]
		[0., 0., 1., 0.]
		);

	let mut rank: Matrix<f64> = [0.2f64,0.4,0.2,0.2].iter().map(|x| x.clone()).to_matrix(1,4).transpose();

	rank = super::pagerank(&adjm, &rank, 1000, 0.85, 0.001); 	
	println!("{},{}",rank,rank.iter().fold(0f64,|acc,b| acc+b));  	
	
	}

}
