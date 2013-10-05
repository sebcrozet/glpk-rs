use std::libc::*;

/* library version numbers: */
pub static GLP_MAJOR_VERSION : c_int = 4;
pub static GLP_MINOR_VERSION : c_int = 52;

/* optimization direction flag: */
pub static GLP_MIN : c_int = 1;  /* minimization */
pub static GLP_MAX : c_int = 2;  /* maximization */

/* kind of structural variable: */
pub static GLP_CV : c_int = 1;  /* continuous variable */
pub static GLP_IV : c_int = 2;  /* integer variable */
pub static GLP_BV : c_int = 3;  /* binary variable */

/* type of auxiliary/structural variable: */
pub static GLP_FR : c_int = 1;  /* free (unbounded) variable */
pub static GLP_LO : c_int = 2;  /* variable with lower bound */
pub static GLP_UP : c_int = 3;  /* variable with upper bound */
pub static GLP_DB : c_int = 4;  /* double-bounded variable */
pub static GLP_FX : c_int = 5;  /* fixed variable */

/* status of auxiliary/structural variable: */
pub static GLP_BS : c_int = 1;  /* basic variable */
pub static GLP_NL : c_int = 2;  /* non-basic variable on lower bound */
pub static GLP_NU : c_int = 3;  /* non-basic variable on upper bound */
pub static GLP_NF : c_int = 4;  /* non-basic free (unbounded) variable */
pub static GLP_NS : c_int = 5;  /* non-basic fixed variable */

/* scaling options: */
pub static GLP_SF_GM : c_int = 0x01;  /* perform geometric mean scaling */
pub static GLP_SF_EQ : c_int = 0x10;  /* perform equilibration scaling */
pub static GLP_SF_2N : c_int = 0x20;  /* round scale factors to power of two */
pub static GLP_SF_SKIP : c_int = 0x40;  /* skip if problem is well scaled */
pub static GLP_SF_AUTO : c_int = 0x80;  /* choose scaling options automatically */

/* solution indicator: */
pub static GLP_SOL : c_int = 1;  /* basic solution */
pub static GLP_IPT : c_int = 2;  /* interior-point solution */
pub static GLP_MIP : c_int = 3;  /* mixed integer solution */

/* solution status: */
pub static GLP_UNDEF : c_int = 1;  /* solution is undefined */
pub static GLP_FEAS : c_int = 2;  /* solution is feasible */
pub static GLP_INFEAS : c_int = 3;  /* solution is infeasible */
pub static GLP_NOFEAS : c_int = 4;  /* no feasible solution exists */
pub static GLP_OPT : c_int = 5;  /* solution is optimal */
pub static GLP_UNBND : c_int = 6;  /* solution is unbounded */

pub static GLP_BF_FT : c_int = 1;  /* LUF + Forrest-Tomlin */
pub static GLP_BF_BG : c_int = 2;  /* LUF + Schur compl. + Bartels-Golub */
pub static GLP_BF_GR : c_int = 3;  /* LUF + Schur compl. + Givens rotation */

pub static GLP_MSG_OFF : c_int = 0;  /* no output */
pub static GLP_MSG_ERR : c_int = 1;  /* warning and error messages only */
pub static GLP_MSG_ON : c_int = 2;  /* normal output */
pub static GLP_MSG_ALL : c_int = 3;  /* full output */
pub static GLP_MSG_DBG : c_int = 4;  /* debug output */

pub static GLP_PRIMAL : c_int = 1;  /* use primal simplex */
pub static GLP_DUALP : c_int = 2;  /* use dual; if it fails, use primal */
pub static GLP_DUAL : c_int = 3;  /* use dual simplex */

pub static GLP_PT_STD : c_int = 0x11;  /* standard (Dantzig rule) */
pub static GLP_PT_PSE : c_int = 0x22;  /* projected steepest edge */

pub static GLP_RT_STD : c_int = 0x11;  /* standard (textbook) */
pub static GLP_RT_HAR : c_int = 0x22;  /* two-pass Harris' ratio test */

pub static GLP_ORD_NONE : c_int = 0;  /* natural (original) ordering */
pub static GLP_ORD_QMD : c_int = 1;  /* quotient minimum degree (QMD) */
pub static GLP_ORD_AMD : c_int = 2;  /* approx. minimum degree (AMD) */
pub static GLP_ORD_SYMAMD : c_int = 3;  /* approx. minimum degree (SYMAMD) */

pub static GLP_BR_FFV : c_int = 1;  /* first fractional variable */
pub static GLP_BR_LFV : c_int = 2;  /* last fractional variable */
pub static GLP_BR_MFV : c_int = 3;  /* most fractional variable */
pub static GLP_BR_DTH : c_int = 4;  /* heuristic by Driebeck and Tomlin */
pub static GLP_BR_PCH : c_int = 5;  /* hybrid pseudocost heuristic */

pub static GLP_BT_DFS : c_int = 1;  /* depth first search */
pub static GLP_BT_BFS : c_int = 2;  /* breadth first search */
pub static GLP_BT_BLB : c_int = 3;  /* best local bound */
pub static GLP_BT_BPH : c_int = 4;  /* best projection heuristic */

pub static GLP_PP_NONE : c_int = 0;  /* disable preprocessing */
pub static GLP_PP_ROOT : c_int = 1;  /* preprocessing only on root level */
pub static GLP_PP_ALL : c_int = 2;  /* preprocessing on all levels */

pub static GLP_RF_REG : c_int = 0;  /* regular constraint */
pub static GLP_RF_LAZY : c_int = 1;  /* "lazy" constraint */
pub static GLP_RF_CUT : c_int = 2;  /* cutting plane constraint */

pub static GLP_RF_GMI : c_int = 1;  /* Gomory's mixed integer cut */
pub static GLP_RF_MIR : c_int = 2;  /* mixed integer rounding cut */
pub static GLP_RF_COV : c_int = 3;  /* mixed cover cut */
pub static GLP_RF_CLQ : c_int = 4;  /* clique cut */

/* enable/disable flag: */
pub static GLP_ON : c_int = 1;  /* enable something */
pub static GLP_OFF : c_int = 0;  /* disable something */

/* reason codes: */
pub static GLP_IROWGEN : c_int = 0x01;  /* request for row generation */
pub static GLP_IBINGO : c_int = 0x02;  /* better integer solution found */
pub static GLP_IHEUR : c_int = 0x03;  /* request for heuristic solution */
pub static GLP_ICUTGEN : c_int = 0x04;  /* request for cut generation */
pub static GLP_IBRANCH : c_int = 0x05;  /* request for branching */
pub static GLP_ISELECT : c_int = 0x06;  /* request for subproblem selection */
pub static GLP_IPREPRO : c_int = 0x07;  /* request for preprocessing */

/* branch selection indicator: */
pub static GLP_NO_BRNCH : c_int = 0;  /* select no branch */
pub static GLP_DN_BRNCH : c_int = 1;  /* select down-branch */
pub static GLP_UP_BRNCH : c_int = 2;  /* select up-branch */

/* return codes: */
pub static GLP_EBADB : c_int = 0x01;  /* invalid basis */
pub static GLP_ESING : c_int = 0x02;  /* singular matrix */
pub static GLP_ECOND : c_int = 0x03;  /* ill-conditioned matrix */
pub static GLP_EBOUND : c_int = 0x04;  /* invalid bounds */
pub static GLP_EFAIL : c_int = 0x05;  /* solver failed */
pub static GLP_EOBJLL : c_int = 0x06;  /* objective lower limit reached */
pub static GLP_EOBJUL : c_int = 0x07;  /* objective upper limit reached */
pub static GLP_EITLIM : c_int = 0x08;  /* iteration limit exceeded */
pub static GLP_ETMLIM : c_int = 0x09;  /* time limit exceeded */
pub static GLP_ENOPFS : c_int = 0x0A;  /* no primal feasible solution */
pub static GLP_ENODFS : c_int = 0x0B;  /* no dual feasible solution */
pub static GLP_EROOT : c_int = 0x0C;  /* root LP optimum not provided */
pub static GLP_ESTOP : c_int = 0x0D;  /* search terminated by application */
pub static GLP_EMIPGAP : c_int = 0x0E;  /* relative mip gap tolerance reached */
pub static GLP_ENOFEAS : c_int = 0x0F;  /* no primal/dual feasible solution */
pub static GLP_ENOCVG : c_int = 0x10;  /* no convergence */
pub static GLP_EINSTAB : c_int = 0x11;  /* numerical instability */
pub static GLP_EDATA : c_int = 0x12;  /* invalid data */
pub static GLP_ERANGE : c_int = 0x13;  /* result out of range */

/* condition indicator: */
pub static GLP_KKT_PE : c_int = 1;  /* primal equalities */
pub static GLP_KKT_PB : c_int = 2;  /* primal bounds */
pub static GLP_KKT_DE : c_int = 3;  /* dual equalities */
pub static GLP_KKT_DB : c_int = 4;  /* dual bounds */
pub static GLP_KKT_CS : c_int = 5;  /* complementary slackness */

/* MPS file format: */
pub static GLP_MPS_DECK : c_int = 1;  /* fixed (ancient) */
pub static GLP_MPS_FILE : c_int = 2;  /* free (modern) */

/* assignment problem formulation: */
pub static GLP_ASN_MIN : c_int = 1;  /* perfect matching (minimization) */
pub static GLP_ASN_MAX : c_int = 2;  /* perfect matching (maximization) */
pub static GLP_ASN_MMP : c_int = 3;  /* maximum matching */

/* automatically generated by rust-bindgen */

pub type Struct_glp_prob = c_void;
pub type glp_prob = Struct_glp_prob;
pub struct glp_bfcp {
    msg_lev: c_int,
    _type: c_int,
    lu_size: c_int,
    piv_tol: c_double,
    piv_lim: c_int,
    suhl: c_int,
    eps_tol: c_double,
    max_gro: c_double,
    nfs_max: c_int,
    upd_tol: c_double,
    nrs_max: c_int,
    rs_size: c_int,
    foo_bar: [c_double, ..38u],
}

pub struct glp_smcp {
    msg_lev: c_int,
    meth: c_int,
    pricing: c_int,
    r_test: c_int,
    tol_bnd: c_double,
    tol_dj: c_double,
    tol_piv: c_double,
    obj_ll: c_double,
    obj_ul: c_double,
    it_lim: c_int,
    tm_lim: c_int,
    out_frq: c_int,
    out_dly: c_int,
    presolve: c_int,
    foo_bar: [c_double, ..36u],
}
pub type Smcp = glp_smcp;

pub struct glp_iptcp {
    msg_lev: c_int,
    ord_alg: c_int,
    foo_bar: [c_double, ..48u],
}

pub type Struct_glp_tree = c_void;
pub type glp_tree = Struct_glp_tree;
pub struct glp_iocp {
    msg_lev: c_int,
    br_tech: c_int,
    bt_tech: c_int,
    tol_int: c_double,
    tol_obj: c_double,
    tm_lim: c_int,
    out_frq: c_int,
    out_dly: c_int,
    cb_func: extern "C" fn(arg1: *mut glp_tree, arg2: *mut c_void),
    cb_info: *mut c_void,
    cb_size: c_int,
    pp_tech: c_int,
    mip_gap: c_double,
    mir_cuts: c_int,
    gmi_cuts: c_int,
    cov_cuts: c_int,
    clq_cuts: c_int,
    presolve: c_int,
    binarize: c_int,
    fp_heur: c_int,
    ps_heur: c_int,
    ps_tm_lim: c_int,
    use_sol: c_int,
    save_sol: *c_schar,
    alien: c_int,
    foo_bar: [c_double, ..25u],
}

pub struct glp_attr {
    level: c_int,
    origin: c_int,
    klass: c_int,
    foo_bar: [c_double, ..7u],
}

pub struct glp_mpscp {
    blank: c_int,
    obj_name: *mut c_schar,
    tol_mps: c_double,
    foo_bar: [c_double, ..17u],
}

pub struct glp_cpxcp {
    foo_bar: [c_double, ..20u],
}

pub type Struct_glp_tran = c_void;
pub type glp_tran = Struct_glp_tran;
pub type glp_errfunc = extern "C" fn(arg1: *c_schar);
pub type glp_graph = Struct_glp_graph;
pub type glp_vertex = Struct_glp_vertex;
pub type glp_arc = Struct_glp_arc;
pub struct Struct_glp_graph {
    pool: *mut c_void,
    name: *mut c_schar,
    nv_max: c_int,
    nv: c_int,
    na: c_int,
    v: *mut *mut glp_vertex,
    index: *mut c_void,
    v_size: c_int,
    a_size: c_int,
}

pub struct Struct_glp_vertex {
    i: c_int,
    name: *mut c_schar,
    entry: *mut c_void,
    data: *mut c_void,
    temp: *mut c_void,
    _in: *mut glp_arc,
    out: *mut glp_arc,
}

pub struct Struct_glp_arc {
    tail: *mut glp_vertex,
    head: *mut glp_vertex,
    data: *mut c_void,
    temp: *mut c_void,
    t_prev: *mut glp_arc,
    t_next: *mut glp_arc,
    h_prev: *mut glp_arc,
    h_next: *mut glp_arc,
}

#[link_args = "-lglpk"]
extern "C" {
    pub fn glp_create_prob() -> *mut glp_prob;
    pub fn glp_set_prob_name(P: *mut glp_prob, name: *c_schar);
    pub fn glp_set_obj_name(P: *mut glp_prob, name: *c_schar);
    pub fn glp_set_obj_dir(P: *mut glp_prob, dir: c_int);
    pub fn glp_add_rows(P: *mut glp_prob, nrs: c_int) -> c_int;
    pub fn glp_add_cols(P: *mut glp_prob, ncs: c_int) -> c_int;
    pub fn glp_set_row_name(P: *mut glp_prob, i: c_int, name: *c_schar);
    pub fn glp_set_col_name(P: *mut glp_prob, j: c_int, name: *c_schar);
    pub fn glp_set_row_bnds(P: *mut glp_prob, i: c_int, _type: c_int,
                            lb: c_double, ub: c_double);
    pub fn glp_set_col_bnds(P: *mut glp_prob, j: c_int, _type: c_int,
                            lb: c_double, ub: c_double);
    pub fn glp_set_obj_coef(P: *mut glp_prob, j: c_int, coef: c_double);
    pub fn glp_set_mat_row(P: *mut glp_prob, i: c_int, len: c_int,
                           ind: *c_int, val: *c_double);
    pub fn glp_set_mat_col(P: *mut glp_prob, j: c_int, len: c_int,
                           ind: *c_int, val: *c_double);
    pub fn glp_load_matrix(P: *mut glp_prob, ne: c_int, ia: *c_int,
                           ja: *c_int, ar: *c_double);
    pub fn glp_check_dup(m: c_int, n: c_int, ne: c_int, ia: *c_int,
                         ja: *c_int) -> c_int;
    pub fn glp_sort_matrix(P: *mut glp_prob);
    pub fn glp_del_rows(P: *mut glp_prob, nrs: c_int, num: *c_int);
    pub fn glp_del_cols(P: *mut glp_prob, ncs: c_int, num: *c_int);
    pub fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob,
                         names: c_int);
    pub fn glp_erase_prob(P: *mut glp_prob);
    pub fn glp_delete_prob(P: *mut glp_prob);
    pub fn glp_get_prob_name(P: *mut glp_prob) -> *c_schar;
    pub fn glp_get_obj_name(P: *mut glp_prob) -> *c_schar;
    pub fn glp_get_obj_dir(P: *mut glp_prob) -> c_int;
    pub fn glp_get_num_rows(P: *mut glp_prob) -> c_int;
    pub fn glp_get_num_cols(P: *mut glp_prob) -> c_int;
    pub fn glp_get_row_name(P: *mut glp_prob, i: c_int) -> *c_schar;
    pub fn glp_get_col_name(P: *mut glp_prob, j: c_int) -> *c_schar;
    pub fn glp_get_row_type(P: *mut glp_prob, i: c_int) -> c_int;
    pub fn glp_get_row_lb(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_get_row_ub(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_get_col_type(P: *mut glp_prob, j: c_int) -> c_int;
    pub fn glp_get_col_lb(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_get_col_ub(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_get_obj_coef(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_get_num_nz(P: *mut glp_prob) -> c_int;
    pub fn glp_get_mat_row(P: *mut glp_prob, i: c_int, ind: *mut c_int,
                           val: *mut c_double) -> c_int;
    pub fn glp_get_mat_col(P: *mut glp_prob, j: c_int, ind: *mut c_int,
                           val: *mut c_double) -> c_int;
    pub fn glp_create_index(P: *mut glp_prob);
    pub fn glp_find_row(P: *mut glp_prob, name: *c_schar) -> c_int;
    pub fn glp_find_col(P: *mut glp_prob, name: *c_schar) -> c_int;
    pub fn glp_delete_index(P: *mut glp_prob);
    pub fn glp_set_rii(P: *mut glp_prob, i: c_int, rii: c_double);
    pub fn glp_set_sjj(P: *mut glp_prob, j: c_int, sjj: c_double);
    pub fn glp_get_rii(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_get_sjj(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_scale_prob(P: *mut glp_prob, flags: c_int);
    pub fn glp_unscale_prob(P: *mut glp_prob);
    pub fn glp_set_row_stat(P: *mut glp_prob, i: c_int, stat: c_int);
    pub fn glp_set_col_stat(P: *mut glp_prob, j: c_int, stat: c_int);
    pub fn glp_std_basis(P: *mut glp_prob);
    pub fn glp_adv_basis(P: *mut glp_prob, flags: c_int);
    pub fn glp_cpx_basis(P: *mut glp_prob);
    pub fn glp_simplex(P: *mut glp_prob, parm: *glp_smcp) -> c_int;
    pub fn glp_exact(P: *mut glp_prob, parm: *glp_smcp) -> c_int;
    pub fn glp_init_smcp(parm: *mut glp_smcp);
    pub fn glp_get_status(P: *mut glp_prob) -> c_int;
    pub fn glp_get_prim_stat(P: *mut glp_prob) -> c_int;
    pub fn glp_get_dual_stat(P: *mut glp_prob) -> c_int;
    pub fn glp_get_obj_val(P: *mut glp_prob) -> c_double;
    pub fn glp_get_row_stat(P: *mut glp_prob, i: c_int) -> c_int;
    pub fn glp_get_row_prim(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_get_row_dual(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_get_col_stat(P: *mut glp_prob, j: c_int) -> c_int;
    pub fn glp_get_col_prim(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_get_col_dual(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_get_unbnd_ray(P: *mut glp_prob) -> c_int;
    pub fn glp_interior(P: *mut glp_prob, parm: *glp_iptcp) -> c_int;
    pub fn glp_init_iptcp(parm: *mut glp_iptcp);
    pub fn glp_ipt_status(P: *mut glp_prob) -> c_int;
    pub fn glp_ipt_obj_val(P: *mut glp_prob) -> c_double;
    pub fn glp_ipt_row_prim(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_ipt_row_dual(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_ipt_col_prim(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_ipt_col_dual(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_set_col_kind(P: *mut glp_prob, j: c_int, kind: c_int);
    pub fn glp_get_col_kind(P: *mut glp_prob, j: c_int) -> c_int;
    pub fn glp_get_num_int(P: *mut glp_prob) -> c_int;
    pub fn glp_get_num_bin(P: *mut glp_prob) -> c_int;
    pub fn glp_intopt(P: *mut glp_prob, parm: *glp_iocp) -> c_int;
    pub fn glp_init_iocp(parm: *mut glp_iocp);
    pub fn glp_mip_status(P: *mut glp_prob) -> c_int;
    pub fn glp_mip_obj_val(P: *mut glp_prob) -> c_double;
    pub fn glp_mip_row_val(P: *mut glp_prob, i: c_int) -> c_double;
    pub fn glp_mip_col_val(P: *mut glp_prob, j: c_int) -> c_double;
    pub fn glp_check_kkt(P: *mut glp_prob, sol: c_int, cond: c_int,
                         ae_max: *mut c_double, ae_ind: *mut c_int,
                         re_max: *mut c_double, re_ind: *mut c_int);
    pub fn glp_print_sol(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_read_sol(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_write_sol(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_print_ranges(P: *mut glp_prob, len: c_int, list: *c_int,
                            flags: c_int, fname: *c_schar) -> c_int;
    pub fn glp_print_ipt(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_read_ipt(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_write_ipt(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_print_mip(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_read_mip(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_write_mip(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_bf_exists(P: *mut glp_prob) -> c_int;
    pub fn glp_factorize(P: *mut glp_prob) -> c_int;
    pub fn glp_bf_updated(P: *mut glp_prob) -> c_int;
    pub fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    pub fn glp_set_bfcp(P: *mut glp_prob, parm: *glp_bfcp);
    pub fn glp_get_bhead(P: *mut glp_prob, k: c_int) -> c_int;
    pub fn glp_get_row_bind(P: *mut glp_prob, i: c_int) -> c_int;
    pub fn glp_get_col_bind(P: *mut glp_prob, j: c_int) -> c_int;
    pub fn glp_ftran(P: *mut glp_prob, x: *mut c_double);
    pub fn glp_btran(P: *mut glp_prob, x: *mut c_double);
    pub fn glp_warm_up(P: *mut glp_prob) -> c_int;
    pub fn glp_eval_tab_row(P: *mut glp_prob, k: c_int, ind: *mut c_int,
                            val: *mut c_double) -> c_int;
    pub fn glp_eval_tab_col(P: *mut glp_prob, k: c_int, ind: *mut c_int,
                            val: *mut c_double) -> c_int;
    pub fn glp_transform_row(P: *mut glp_prob, len: c_int, ind: *mut c_int,
                             val: *mut c_double) -> c_int;
    pub fn glp_transform_col(P: *mut glp_prob, len: c_int, ind: *mut c_int,
                             val: *mut c_double) -> c_int;
    pub fn glp_prim_rtest(P: *mut glp_prob, len: c_int, ind: *c_int,
                          val: *c_double, dir: c_int, eps: c_double) -> c_int;
    pub fn glp_dual_rtest(P: *mut glp_prob, len: c_int, ind: *c_int,
                          val: *c_double, dir: c_int, eps: c_double) -> c_int;
    pub fn glp_analyze_bound(P: *mut glp_prob, k: c_int,
                             value1: *mut c_double, var1: *mut c_int,
                             value2: *mut c_double, var2: *mut c_int);
    pub fn glp_analyze_coef(P: *mut glp_prob, k: c_int, coef1: *mut c_double,
                            var1: *mut c_int, value1: *mut c_double,
                            coef2: *mut c_double, var2: *mut c_int,
                            value2: *mut c_double);
    pub fn glp_ios_reason(T: *mut glp_tree) -> c_int;
    pub fn glp_ios_get_prob(T: *mut glp_tree) -> *mut glp_prob;
    pub fn glp_ios_tree_size(T: *mut glp_tree, a_cnt: *mut c_int,
                             n_cnt: *mut c_int, t_cnt: *mut c_int);
    pub fn glp_ios_curr_node(T: *mut glp_tree) -> c_int;
    pub fn glp_ios_next_node(T: *mut glp_tree, p: c_int) -> c_int;
    pub fn glp_ios_prev_node(T: *mut glp_tree, p: c_int) -> c_int;
    pub fn glp_ios_up_node(T: *mut glp_tree, p: c_int) -> c_int;
    pub fn glp_ios_node_level(T: *mut glp_tree, p: c_int) -> c_int;
    pub fn glp_ios_node_bound(T: *mut glp_tree, p: c_int) -> c_double;
    pub fn glp_ios_best_node(T: *mut glp_tree) -> c_int;
    pub fn glp_ios_mip_gap(T: *mut glp_tree) -> c_double;
    pub fn glp_ios_node_data(T: *mut glp_tree, p: c_int) -> *mut c_void;
    pub fn glp_ios_row_attr(T: *mut glp_tree, i: c_int, attr: *mut glp_attr);
    pub fn glp_ios_pool_size(T: *mut glp_tree) -> c_int;
    pub fn glp_ios_add_row(T: *mut glp_tree, name: *c_schar, klass: c_int,
                           flags: c_int, len: c_int, ind: *c_int, val: *c_double,
                           _type: c_int, rhs: c_double) -> c_int;
    pub fn glp_ios_del_row(T: *mut glp_tree, i: c_int);
    pub fn glp_ios_clear_pool(T: *mut glp_tree);
    pub fn glp_ios_can_branch(T: *mut glp_tree, j: c_int) -> c_int;
    pub fn glp_ios_branch_upon(T: *mut glp_tree, j: c_int, sel: c_int);
    pub fn glp_ios_select_node(T: *mut glp_tree, p: c_int);
    pub fn glp_ios_heur_sol(T: *mut glp_tree, x: *c_double) -> c_int;
    pub fn glp_ios_terminate(T: *mut glp_tree);
    pub fn glp_init_mpscp(parm: *mut glp_mpscp);
    pub fn glp_read_mps(P: *mut glp_prob, fmt: c_int, parm: *glp_mpscp,
                        fname: *c_schar) -> c_int;
    pub fn glp_write_mps(P: *mut glp_prob, fmt: c_int, parm: *glp_mpscp,
                         fname: *c_schar) -> c_int;
    pub fn glp_init_cpxcp(parm: *mut glp_cpxcp);
    pub fn glp_read_lp(P: *mut glp_prob, parm: *glp_cpxcp, fname: *c_schar) ->
     c_int;
    pub fn glp_write_lp(P: *mut glp_prob, parm: *glp_cpxcp, fname: *c_schar)
     -> c_int;
    pub fn glp_read_prob(P: *mut glp_prob, flags: c_int, fname: *c_schar) ->
     c_int;
    pub fn glp_write_prob(P: *mut glp_prob, flags: c_int, fname: *c_schar) ->
     c_int;
    pub fn glp_mpl_alloc_wksp() -> *mut glp_tran;
    pub fn glp_mpl_read_model(tran: *mut glp_tran, fname: *c_schar,
                              skip: c_int) -> c_int;
    pub fn glp_mpl_read_data(tran: *mut glp_tran, fname: *c_schar) -> c_int;
    pub fn glp_mpl_generate(tran: *mut glp_tran, fname: *c_schar) -> c_int;
    pub fn glp_mpl_build_prob(tran: *mut glp_tran, prob: *mut glp_prob);
    pub fn glp_mpl_postsolve(tran: *mut glp_tran, prob: *mut glp_prob,
                             sol: c_int) -> c_int;
    pub fn glp_mpl_free_wksp(tran: *mut glp_tran);
    // pub fn glp_main(argc: c_int, argv: c_void) -> c_int;
    pub fn glp_read_cnfsat(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_check_cnfsat(P: *mut glp_prob) -> c_int;
    pub fn glp_write_cnfsat(P: *mut glp_prob, fname: *c_schar) -> c_int;
    pub fn glp_minisat1(P: *mut glp_prob) -> c_int;
    pub fn glp_intfeas1(P: *mut glp_prob, use_bound: c_int, obj_bound: c_int)
     -> c_int;
    pub fn glp_init_env() -> c_int;
    pub fn glp_version() -> *c_schar;
    pub fn glp_free_env() -> c_int;
    pub fn glp_puts(s: *c_schar);
    pub fn glp_printf(fmt: *c_schar);
    pub fn glp_term_out(flag: c_int) -> c_int;
    // pub fn glp_term_hook(func:
    //                          extern "C" fn(arg1: *mut c_void, arg2: *c_schar)
    //                              -> c_int, info: *mut c_void);
    pub fn glp_open_tee(name: *c_schar) -> c_int;
    pub fn glp_close_tee() -> c_int;
    pub fn glp_error_(file: *c_schar, line: c_int) -> glp_errfunc;
    pub fn glp_assert_(expr: *c_schar, file: *c_schar, line: c_int);
    // pub fn glp_error_hook(func: extern "C" fn(arg1: *mut c_void),
    //                       info: *mut c_void);
    // pub fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    // pub fn glp_realloc(ptr: *mut c_void, n: c_int, size: c_int) ->
    //  *mut c_void;
    // pub fn glp_free(ptr: *mut c_void);
    pub fn glp_mem_limit(limit: c_int);
    pub fn glp_mem_usage(count: *mut c_int, cpeak: *mut c_int,
                         total: *mut size_t, tpeak: *mut size_t);
    pub fn glp_create_graph(v_size: c_int, a_size: c_int) -> *mut glp_graph;
    pub fn glp_set_graph_name(G: *mut glp_graph, name: *c_schar);
    pub fn glp_add_vertices(G: *mut glp_graph, nadd: c_int) -> c_int;
    pub fn glp_set_vertex_name(G: *mut glp_graph, i: c_int, name: *c_schar);
    pub fn glp_add_arc(G: *mut glp_graph, i: c_int, j: c_int) -> *mut glp_arc;
    pub fn glp_del_vertices(G: *mut glp_graph, ndel: c_int, num: *c_int);
    pub fn glp_del_arc(G: *mut glp_graph, a: *mut glp_arc);
    pub fn glp_erase_graph(G: *mut glp_graph, v_size: c_int, a_size: c_int);
    pub fn glp_delete_graph(G: *mut glp_graph);
    pub fn glp_create_v_index(G: *mut glp_graph);
    pub fn glp_find_vertex(G: *mut glp_graph, name: *c_schar) -> c_int;
    pub fn glp_delete_v_index(G: *mut glp_graph);
    pub fn glp_read_graph(G: *mut glp_graph, fname: *c_schar) -> c_int;
    pub fn glp_write_graph(G: *mut glp_graph, fname: *c_schar) -> c_int;
    pub fn glp_mincost_lp(P: *mut glp_prob, G: *mut glp_graph, names: c_int,
                          v_rhs: c_int, a_low: c_int, a_cap: c_int,
                          a_cost: c_int);
    pub fn glp_mincost_okalg(G: *mut glp_graph, v_rhs: c_int, a_low: c_int,
                             a_cap: c_int, a_cost: c_int, sol: *mut c_double,
                             a_x: c_int, v_pi: c_int) -> c_int;
    pub fn glp_mincost_relax4(G: *mut glp_graph, v_rhs: c_int, a_low: c_int,
                              a_cap: c_int, a_cost: c_int, crash: c_int,
                              sol: *mut c_double, a_x: c_int, a_rc: c_int) ->
     c_int;
    pub fn glp_maxflow_lp(P: *mut glp_prob, G: *mut glp_graph, names: c_int,
                          s: c_int, t: c_int, a_cap: c_int);
    pub fn glp_maxflow_ffalg(G: *mut glp_graph, s: c_int, t: c_int,
                             a_cap: c_int, sol: *mut c_double, a_x: c_int,
                             v_cut: c_int) -> c_int;
    pub fn glp_check_asnprob(G: *mut glp_graph, v_set: c_int) -> c_int;
    pub fn glp_asnprob_lp(P: *mut glp_prob, form: c_int, G: *mut glp_graph,
                          names: c_int, v_set: c_int, a_cost: c_int) -> c_int;
    pub fn glp_asnprob_okalg(form: c_int, G: *mut glp_graph, v_set: c_int,
                             a_cost: c_int, sol: *mut c_double, a_x: c_int) ->
     c_int;
    pub fn glp_asnprob_hall(G: *mut glp_graph, v_set: c_int, a_x: c_int) ->
     c_int;
    pub fn glp_cpp(G: *mut glp_graph, v_t: c_int, v_es: c_int, v_ls: c_int) ->
     c_double;
    pub fn glp_read_mincost(G: *mut glp_graph, v_rhs: c_int, a_low: c_int,
                            a_cap: c_int, a_cost: c_int, fname: *c_schar) ->
     c_int;
    pub fn glp_write_mincost(G: *mut glp_graph, v_rhs: c_int, a_low: c_int,
                             a_cap: c_int, a_cost: c_int, fname: *c_schar) ->
     c_int;
    pub fn glp_read_maxflow(G: *mut glp_graph, s: *mut c_int, t: *mut c_int,
                            a_cap: c_int, fname: *c_schar) -> c_int;
    pub fn glp_write_maxflow(G: *mut glp_graph, s: c_int, t: c_int,
                             a_cap: c_int, fname: *c_schar) -> c_int;
    pub fn glp_read_asnprob(G: *mut glp_graph, v_set: c_int, a_cost: c_int,
                            fname: *c_schar) -> c_int;
    pub fn glp_write_asnprob(G: *mut glp_graph, v_set: c_int, a_cost: c_int,
                             fname: *c_schar) -> c_int;
    pub fn glp_read_ccdata(G: *mut glp_graph, v_wgt: c_int, fname: *c_schar)
     -> c_int;
    pub fn glp_write_ccdata(G: *mut glp_graph, v_wgt: c_int, fname: *c_schar)
     -> c_int;
    pub fn glp_netgen(G: *mut glp_graph, v_rhs: c_int, a_cap: c_int,
                      a_cost: c_int, parm: [c_int, ..16u]) -> c_int;
    pub fn glp_netgen_prob(nprob: c_int, parm: [c_int, ..16u]);
    pub fn glp_gridgen(G: *mut glp_graph, v_rhs: c_int, a_cap: c_int,
                       a_cost: c_int, parm: [c_int, ..15u]) -> c_int;
    pub fn glp_rmfgen(G: *mut glp_graph, s: *mut c_int, t: *mut c_int,
                      a_cap: c_int, parm: [c_int, ..6u]) -> c_int;
    pub fn glp_weak_comp(G: *mut glp_graph, v_num: c_int) -> c_int;
    pub fn glp_strong_comp(G: *mut glp_graph, v_num: c_int) -> c_int;
    pub fn glp_top_sort(G: *mut glp_graph, v_num: c_int) -> c_int;
    pub fn glp_wclique_exact(G: *mut glp_graph, v_wgt: c_int,
                             sol: *mut c_double, v_set: c_int) -> c_int;
}
