use std::vec::raw;
use std::str;
use std::ptr;
use std::libc::*;
use ffi::*;

unsafe fn p<T>(v: &[T]) -> *T {
    raw::to_ptr(v)
}

unsafe fn mp<T>(v: &mut [T]) -> *mut T {
    raw::to_mut_ptr(v)
}

pub struct Prob {
    priv this: *mut glp_prob
}

impl Prob {
    #[inline(never)] #[fixed_stack_segment]
    pub fn new() -> Prob {
        Prob {
            this: unsafe { glp_create_prob() }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_name(&mut self, name: &str) {
        do name.with_c_str |c_name| {
            unsafe {
                glp_set_prob_name(self.this, c_name);
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_obj_name(&mut self, name: &str) {
        do name.with_c_str |c_name| {
            unsafe {
                glp_set_obj_name(self.this, c_name);
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_obj_dir(&mut self, dir: c_int) {
        unsafe { glp_set_obj_dir(self.this, dir) }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn add_rows(&mut self, nrs: c_int) -> c_int {
        unsafe { glp_add_rows(self.this, nrs) }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn add_cols(&mut self, ncs: c_int) -> c_int {
        unsafe { glp_add_cols(self.this, ncs) }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_row_name(&mut self, i: c_int, name: &str) {
        do name.with_c_str |c_name| {
            unsafe {
                glp_set_row_name(self.this, i, c_name);
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_col_name(&mut self, j: c_int, name: &str) {
        do name.with_c_str |c_name| {
            unsafe {
                glp_set_col_name(self.this, j, c_name);
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_row_bnds(&mut self, i: c_int, _type: c_int, lb: c_double, ub: c_double) {
        unsafe { glp_set_row_bnds(self.this, i, _type, lb, ub) }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_col_bnds(&mut self, j: c_int, _type: c_int, lb: c_double, ub: c_double) {
        unsafe { glp_set_col_bnds(self.this, j, _type, lb, ub) }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_obj_coef(&mut self, j: c_int, coef: c_double) {
        unsafe { glp_set_obj_coef(self.this, j, coef) }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_mat_row(&mut self, i: c_int, len: c_int, ind: &[c_int], val: &[c_double]) {
        unsafe {
            glp_set_mat_row(self.this, i, len, p(ind), p(val))
        }
    } 

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_mat_col(&mut self, j: c_int, len: c_int, ind: &[c_int], val: &[c_double]) {
        unsafe {
            glp_set_mat_col(self.this, j, len, p(ind), p(val))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn load_matrix(&mut self, ne: c_int, ia: &[c_int], ja: &[c_int], ar: &[c_double]) {
        unsafe {
            glp_load_matrix(self.this, ne, p(ia), p(ja), p(ar))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn sort_matrix(&mut self) {
        unsafe {
            glp_sort_matrix(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn del_rows(&mut self, nrs: c_int, num: &[c_int]) {
        unsafe {
            glp_del_rows(self.this, nrs, p(num))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn del_cols(&mut self, ncs: c_int, num: &[c_int]) {
        unsafe {
            glp_del_cols(self.this, ncs, p(num))
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn copy(&self, dest: &mut Prob, names: c_int) {
        unsafe {
            glp_copy_prob(dest.this, self.this, names)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn erase(&mut self) {
        unsafe {
            glp_erase_prob(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_name(&self) -> ~str {
        unsafe {
            let c_str = glp_get_prob_name(self.this);

            if c_str.is_null() {
                ~""
            }
            else {
                str::raw::from_c_str(c_str)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_obj_name(&self) -> ~str {
        unsafe {
            let c_str = glp_get_obj_name(self.this);

            if c_str.is_null() {
                ~""
            }
            else {
                str::raw::from_c_str(c_str)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_obj_dir(&self) -> c_int {
        unsafe {
            glp_get_obj_dir(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_num_rows(&self) -> c_int {
        unsafe {
            glp_get_num_rows(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_num_cols(&self) -> c_int {
        unsafe {
            glp_get_num_cols(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_name(&self, i: c_int) -> ~str {
        unsafe {
            let c_str = glp_get_row_name(self.this, i);

            if c_str.is_null() {
                ~""
            }
            else {
                str::raw::from_c_str(c_str)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_name(&self, j: c_int) -> ~str {
        unsafe {
            let c_str = glp_get_col_name(self.this, j);

            if c_str.is_null() {
                ~""
            }
            else {
                str::raw::from_c_str(c_str)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_type(&self, i: c_int) -> c_int {
        unsafe {
            glp_get_row_type(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_lb(&self, i: c_int) -> c_double {
        unsafe {
            glp_get_row_lb(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_ub(&self, i: c_int) -> c_double {
        unsafe {
            glp_get_row_ub(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_type(&self, j: c_int) -> c_int {
        unsafe {
            glp_get_col_type(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_lb(&self, j: c_int) -> c_double {
        unsafe {
            glp_get_col_lb(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_ub(&self, j: c_int) -> c_double {
        unsafe {
            glp_get_col_ub(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_obj_coef(&self, j: c_int) -> c_double {
        unsafe {
            glp_get_obj_coef(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_num_nz(&self) -> c_int {
        unsafe {
            glp_get_num_nz(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_mat_row(&self, i: c_int, ind: &mut [c_int], val: &mut [c_double]) -> c_int {
        // FIXME: this is unsafe
        unsafe {
            glp_get_mat_row(self.this, i, mp(ind), mp(val))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_mat_col(&self, j: c_int, ind: &mut [c_int], val: &mut [c_double]) -> c_int {
        // FIXME: this is unsafe
        unsafe {
            glp_get_mat_col(self.this, j, mp(ind), mp(val))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn create_index(&mut self) {
        unsafe {
            glp_create_index(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn find_row(&mut self, name: &str) -> c_int {
        unsafe {
            do name.with_c_str |c_str| {
                glp_find_row(self.this, c_str)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn find_col(&mut self, name: &str) -> c_int {
        unsafe {
            do name.with_c_str |c_str| {
                glp_find_col(self.this, c_str)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn delete_index(&mut self) {
        unsafe {
            glp_delete_index(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_rii(&mut self, i: c_int, rii: c_double) {
        unsafe {
            glp_set_rii(self.this, i, rii)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_sjj(&mut self, j: c_int, sjj: c_double) {
        unsafe {
            glp_set_sjj(self.this, j, sjj)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_rii(&self, i: c_int) -> c_double {
        unsafe {
            glp_get_rii(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_sjj(&self, j: c_int) -> c_double {
        unsafe {
            glp_get_sjj(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn scale(&mut self, flags: c_int) {
        unsafe {
            glp_scale_prob(self.this, flags)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn unscale(&mut self) {
        unsafe {
            glp_unscale_prob(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_row_stat(&mut self, i: c_int, stat: c_int) {
        unsafe {
            glp_set_row_stat(self.this, i, stat)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_col_stat(&mut self, j: c_int, stat: c_int) {
        unsafe {
            glp_set_col_stat(self.this, j, stat)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn std_basis(&mut self) {
        unsafe {
            glp_std_basis(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn adv_basis(&mut self, flags: c_int) {
        unsafe {
            glp_adv_basis(self.this, flags)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn cpx_basis(&mut self) {
        unsafe {
            glp_cpx_basis(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn simplex(&mut self, parm: Option<&Smcp>) -> c_int {
        unsafe {
            match parm {
                None    => glp_simplex(self.this, ptr::null()),
                Some(s) => glp_simplex(self.this, ptr::to_unsafe_ptr(s))
            }
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn exact(&mut self, parm: Option<&Smcp>) -> c_int {
        unsafe {
            match parm {
                None    => glp_exact(self.this, ptr::null()),
                Some(s) => glp_exact(self.this, ptr::to_unsafe_ptr(s))
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_status(&self) -> c_int {
        unsafe {
            glp_get_status(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_prim_stat(&self) -> c_int {
        unsafe {
            glp_get_prim_stat(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_dual_stat(&self) -> c_int {
        unsafe {
            glp_get_dual_stat(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_obj_val(&self) -> c_double {
        unsafe {
            glp_get_obj_val(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_stat(&self, i: c_int) -> c_int {
        unsafe {
            glp_get_row_stat(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_prim(&self, i: c_int) -> c_double {
        unsafe {
            glp_get_row_prim(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_dual(&self, i: c_int) -> c_double {
        unsafe {
            glp_get_row_dual(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_stat(&self, j: c_int) -> c_int {
        unsafe {
            glp_get_col_stat(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_prim(&self, j: c_int) -> c_double {
        unsafe {
            glp_get_col_prim(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_dual(&self, j: c_int) -> c_double {
        unsafe {
            glp_get_col_dual(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_unbnd_ray(&self) -> c_int {
        unsafe {
            glp_get_unbnd_ray(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn interior(&mut self, parm: Option<&Iptcp>) -> c_int {
        unsafe {
            match parm {
                None    => glp_interior(self.this, ptr::null()),
                Some(s) => glp_interior(self.this, ptr::to_unsafe_ptr(s))
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn ipt_status(&mut self) -> c_int {
        unsafe {
            glp_ipt_status(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn ipt_obj_val(&mut self) -> c_double {
        unsafe {
            glp_ipt_obj_val(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn ipt_row_prim(&mut self, i: c_int) -> c_double {
        unsafe {
            glp_ipt_row_prim(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn ipt_row_dual(&mut self, i: c_int) -> c_double {
        unsafe {
            glp_ipt_row_dual(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn ipt_col_prim(&mut self, j: c_int) -> c_double {
        unsafe {
            glp_ipt_col_prim(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn ipt_col_dual(&mut self, j: c_int) -> c_double {
        unsafe {
            glp_ipt_col_dual(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_col_kind(&mut self, j: c_int, kind: c_int) {
        unsafe {
            glp_set_col_kind(self.this, j, kind)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_kind(&self, j: c_int) -> c_int {
        unsafe {
            glp_get_col_kind(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_num_int(&self) -> c_int {
        unsafe {
            glp_get_num_int(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_num_bin(&self) -> c_int {
        unsafe {
            glp_get_num_bin(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn intopt(&mut self, parm: Option<&Iocp>) -> c_int {
        unsafe {
            match parm {
                None    => glp_intopt(self.this, ptr::null()),
                Some(s) => glp_intopt(self.this, ptr::to_unsafe_ptr(s))
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn mip_status(&self) -> c_int {
        unsafe {
            glp_mip_status(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn mip_obj_val(&self) -> c_double {
        unsafe {
            glp_mip_obj_val(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn mip_row_val(&self, i: c_int) -> c_double {
        unsafe {
            glp_mip_row_val(self.this, i)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn mip_col_val(&self, j: c_int) -> c_double {
        unsafe {
            glp_mip_col_val(self.this, j)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn check_kkt(&mut self,
                     sol:    c_int,
                     cond:   c_int,
                     ae_max: &mut c_double,
                     ae_ind: &mut c_int,
                     re_max: &mut c_double,
                     re_ind: &mut c_int) {
        unsafe {
            glp_check_kkt(self.this, sol, cond, ptr::to_mut_unsafe_ptr(ae_max),
                          ptr::to_mut_unsafe_ptr(ae_ind), ptr::to_mut_unsafe_ptr(re_max),
                          ptr::to_mut_unsafe_ptr(re_ind));
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn print_sol(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_print_sol(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn read_sol(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_read_sol(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn write_sol(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_write_sol(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn print_ranges(&mut self, len: c_int, list: &[c_int], flags: c_int, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_print_ranges(self.this, len, p(list), flags, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn print_ipt(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_print_ipt(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn read_ipt(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_read_ipt(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn write_ipt(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_write_ipt(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn print_mip(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_print_mip(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn read_mip(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_read_mip(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn write_mip(&mut self, fname: &str) -> c_int {
        unsafe {
            do fname.with_c_str |c| {
                glp_write_mip(self.this, c)
            }
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn bf_exists(&self) -> c_int {
        unsafe {
            glp_bf_exists(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn factorize(&mut self) -> c_int {
        unsafe {
            glp_factorize(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn bf_updated(&mut self) -> c_int {
        unsafe {
            glp_bf_updated(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_bfcp(&self, parm: &mut Bfcp) {
        unsafe {
            glp_get_bfcp(self.this, ptr::to_mut_unsafe_ptr(parm))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn set_bfcp(&mut self, parm: &Bfcp) {
        unsafe {
            glp_set_bfcp(self.this, ptr::to_unsafe_ptr(parm))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_bhead(&self, k: c_int) -> c_int {
        unsafe {
            glp_get_bhead(self.this, k)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_row_bind(&self, i: c_int) -> c_int {
        unsafe {
            glp_get_row_bind(self.this, i)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn get_col_bind(&self, j: c_int) -> c_int {
        unsafe {
            glp_get_col_bind(self.this, j)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn ftran(&mut self, x: &mut [c_double]) {
        unsafe {
            glp_ftran(self.this, mp(x))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn btran(&mut self, x: &mut [c_double]) {
        unsafe {
            glp_btran(self.this, mp(x))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn warm_up(&mut self) -> c_int {
        unsafe {
            glp_warm_up(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn eval_tab_row(&mut self, k: c_int, ind: &mut [c_int], val: &mut [c_double]) -> c_int {
        unsafe {
            glp_eval_tab_row(self.this, k, mp(ind), mp(val))
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn eval_tab_col(&mut self, k: c_int, ind: &mut [c_int], val: &mut [c_double]) -> c_int {
        unsafe {
            glp_eval_tab_col(self.this, k, mp(ind), mp(val))
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn transform_row(&mut self, len: c_int, ind: &mut [c_int], val: &mut [c_double]) -> c_int {
        unsafe {
            glp_transform_row(self.this, len, mp(ind), mp(val))
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn transform_col(&mut self, len: c_int, ind: &mut [c_int], val: &mut [c_double]) -> c_int {
        unsafe {
            glp_transform_col(self.this, len, mp(ind), mp(val))
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn prim_rtest(&mut self, len: c_int, ind: &[c_int], val: &[c_double], dir: c_int, eps: c_double) -> c_int {
        unsafe {
            glp_prim_rtest(self.this, len, p(ind), p(val), dir, eps)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn dual_rtest(&mut self, len: c_int, ind: &[c_int], val: &[c_double], dir: c_int, eps: c_double) -> c_int {
        unsafe {
            glp_dual_rtest(self.this, len, p(ind), p(val), dir, eps)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn analyze_bound(&mut self,
                         k:      c_int,
                         value1: &mut c_double,
                         var1:   &mut c_int,
                         value2: &mut c_double,
                         var2:   &mut c_int) {
        unsafe {
            glp_analyze_bound(self.this, k, ptr::to_mut_unsafe_ptr(value1),
                              ptr::to_mut_unsafe_ptr(var1), ptr::to_mut_unsafe_ptr(value2),
                              ptr::to_mut_unsafe_ptr(var2))
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn analyze_coef(&mut self,
                        k:      c_int,
                        coef1:  &mut c_double,
                        var1:   &mut c_int,
                        value1: &mut c_double,
                        coef2:  &mut c_double,
                        var2:   &mut c_int,
                        value2: &mut c_double) {
        unsafe {
            glp_analyze_coef(
                self.this, k,
                ptr::to_mut_unsafe_ptr(coef1),
                ptr::to_mut_unsafe_ptr(var1),
                ptr::to_mut_unsafe_ptr(value1),
                ptr::to_mut_unsafe_ptr(coef2),
                ptr::to_mut_unsafe_ptr(var2),
                ptr::to_mut_unsafe_ptr(value2))
        }
    }

    /*
    #[inline(never)] #[fixed_stack_segment]
    pub fn read_mps(&mut self, fmt: c_int, parm: *Mpscp, fname: *schar) -> c_int {
        unsafe {
            glp_read_mps(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn write_mps(&mut self, fmt: c_int, parm: *Mpscp, fname: *schar) -> c_int {
        unsafe {
            glp_write_mps(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn read_lp(&mut self, parm: *Cpxcp, fname: *schar) -> c_int {
        unsafe {
            glp_read_lp(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn write_lp(&mut self, parm: *Cpxcp, fname: *schar) -> c_int {
        unsafe {
            glp_write_lp(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn read(&mut self, flags: c_int, fname: *schar) -> c_int {
        unsafe {
            glp_read(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn write(&mut self, flags: c_int, fname: *schar) -> c_int {
        unsafe {
            glp_write(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn read_cnfsat(&mut self, fname: *schar) -> c_int {
        unsafe {
            glp_read_cnfsat(self.this)
        }

    }
    */

    #[inline(never)] #[fixed_stack_segment]
    pub fn check_cnfsat(&self) -> c_int {
        unsafe {
            glp_check_cnfsat(self.this)
        }

    }

    /*
    #[inline(never)] #[fixed_stack_segment]
    pub fn write_cnfsat(&mut self, fname: *schar) -> c_int {
        unsafe {
            glp_write_cnfsat(self.this)
        }
    }
    */

    #[inline(never)] #[fixed_stack_segment]
    pub fn minisat1(&self) -> c_int {
        unsafe {
            glp_minisat1(self.this)
        }
    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn intfeas1(&mut self, use_bound: c_int, obj_bound: c_int) -> c_int {
        unsafe {
            glp_intfeas1(self.this, use_bound, obj_bound)
        }
    }

    /*
    #[inline(never)] #[fixed_stack_segment]
    pub fn mincost_lp(&mut self, G: *mut Graph, names: c_int, v_rhs: c_int, a_low: c_int, a_cap: c_int, a_cost: c_int) {
        unsafe {
            glp_mincost_lp(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn maxflow_lp(&mut self, G: *mut Graph, names: c_int, s: c_int, t: c_int, a_cap: c_int) {
        unsafe {
            glp_maxflow_lp(self.this)
        }

    }

    #[inline(never)] #[fixed_stack_segment]
    pub fn asnprob_lp(&mut self, form: c_int, G: *mut Graph, names: c_int, v_set: c_int, a_cost: c_int) -> c_int {
        unsafe {
            glp_asnprob_lp(self.this)
        }

    }
    */
}

impl Drop for Prob {
    #[inline(never)] #[fixed_stack_segment]
    fn drop(&mut self) {
        unsafe { glp_delete_prob(self.this); }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ffi::*;

    #[test]
    fn test_new_prob() {
        let mut a = Prob::new();

        a.set_name("name");

        let b = a;

        assert!(b.get_name() == ~"name")
    }

    #[test]
    fn test_small_problem() {
        /*
         * Solve:
         *  maximize	0.6 x + 0.5 y
         *  subject to	x     + 2 y <= 1
         *              3 x   + y <= 2
        */

        /* declare variables */
        let mut lp = Prob::new();

        let mut ia = [0i32, .. 1 + 1000];
        let mut ja = [0i32, .. 1 + 1000];
        let mut ar = [0.0f64, .. 1 + 1000];
        let z;
        let x1;
        let x2;

        /* create problem */
        lp.set_name("short problem");
        lp.set_obj_dir(GLP_MAX);

        /* fill problem */
        lp.add_rows(2);
        lp.set_row_name(1, "p");
        lp.set_row_bnds(1, GLP_UP, 0.0, 1.0);
        lp.set_row_name(2, "q");
        lp.set_row_bnds(2, GLP_UP, 0.0, 2.0);
        lp.add_cols(2);
        lp.set_col_name(1, "x1");
        lp.set_col_bnds(1, GLP_LO, 0.0, 0.0);
        lp.set_obj_coef(1, 0.6);
        lp.set_col_name(2, "x2");
        lp.set_col_bnds(2, GLP_LO, 0.0, 0.0);
        lp.set_obj_coef(2, 0.5);

        ia[1] = 1; ja[1] = 1; ar[1] = 1.0;
        ia[2] = 1; ja[2] = 2; ar[2] = 2.0;
        ia[3] = 2; ja[3] = 1; ar[3] = 3.0;
        ia[4] = 2; ja[4] = 2; ar[4] = 1.0;

        lp.load_matrix(4, ia, ja, ar);

        /* solve problem */
        lp.simplex(None);

        /* recover and display results */
        z  = lp.get_obj_val();
        x1 = lp.get_col_prim(1);
        x2 = lp.get_col_prim(2);

        println!("z = { }; x1 = { }; x2 = { }", z, x1, x2);

        assert!(z.approx_eq(&0.46));
        assert!(x1.approx_eq(&0.6));
        assert!(x2.approx_eq(&0.2));
    }
}
