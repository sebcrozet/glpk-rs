/*
    pub fn glp_asnprob_okalg(form: c_int, G: *mut Graph, v_set: c_int,
                             a_cost: c_int, sol: *mut c_double, a_x: c_int) ->
     c_int;
    pub fn glp_asnprob_hall(G: *mut Graph, v_set: c_int, a_x: c_int) ->
     c_int;
    pub fn glp_cpp(G: *mut Graph, v_t: c_int, v_es: c_int, v_ls: c_int) ->
     c_double;
    pub fn glp_read_mincost(G: *mut Graph, v_rhs: c_int, a_low: c_int,
                            a_cap: c_int, a_cost: c_int, fname: *schar) ->
     c_int;
    pub fn glp_write_mincost(G: *mut Graph, v_rhs: c_int, a_low: c_int,
                             a_cap: c_int, a_cost: c_int, fname: *schar) ->
     c_int;
    pub fn glp_read_maxflow(G: *mut Graph, s: *mut c_int, t: *mut c_int,
                            a_cap: c_int, fname: *schar) -> c_int;
    pub fn glp_write_maxflow(G: *mut Graph, s: c_int, t: c_int,
                             a_cap: c_int, fname: *schar) -> c_int;
    pub fn glp_read_asnprob(G: *mut Graph, v_set: c_int, a_cost: c_int,
                            fname: *schar) -> c_int;
    pub fn glp_write_asnprob(G: *mut Graph, v_set: c_int, a_cost: c_int,
                             fname: *schar) -> c_int;
    pub fn glp_read_ccdata(G: *mut Graph, v_wgt: c_int, fname: *schar)
     -> c_int;
    pub fn glp_write_ccdata(G: *mut Graph, v_wgt: c_int, fname: *schar)
     -> c_int;
    pub fn glp_netgen(G: *mut Graph, v_rhs: c_int, a_cap: c_int,
                      a_cost: c_int, parm: [c_int, ..16u]) -> c_int;
    pub fn glp_netgen_prob(nprob: c_int, parm: [c_int, ..16u]);
    pub fn glp_gridgen(G: *mut Graph, v_rhs: c_int, a_cap: c_int,
                       a_cost: c_int, parm: [c_int, ..15u]) -> c_int;
    pub fn glp_rmfgen(G: *mut Graph, s: *mut c_int, t: *mut c_int,
                      a_cap: c_int, parm: [c_int, ..6u]) -> c_int;
    pub fn glp_weak_comp(G: *mut Graph, v_num: c_int) -> c_int;
    pub fn glp_strong_comp(G: *mut Graph, v_num: c_int) -> c_int;
    pub fn glp_top_sort(G: *mut Graph, v_num: c_int) -> c_int;
    pub fn glp_wclique_exact(G: *mut Graph, v_wgt: c_int,
                             sol: *mut c_double, v_set: c_int) -> c_int;
}



    pub fn glp_check_dup(m: c_int, n: c_int, ne: c_int, ia: void,
                         ja: void) -> c_int;
    pub fn glp_init_smcp(parm: *mut Smcp);
    pub fn glp_init_iptcp(parm: *mut Iptcp);
    pub fn glp_init_iocp(parm: *mut Iocp);
    pub fn glp_ios_reason(T: *mut glp_tree) -> c_int;
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
    pub fn glp_ios_node_data(T: *mut glp_tree, p: c_int) -> *mut void;
    pub fn glp_ios_row_attr(T: *mut glp_tree, i: c_int, attr: *mut Attr);
    pub fn glp_ios_pool_size(T: *mut glp_tree) -> c_int;
    pub fn glp_ios_add_row(T: *mut glp_tree, name: *schar, klass: c_int,
                           flags: c_int, len: c_int, ind: void, val: void,
                           _type: c_int, rhs: c_double) -> c_int;
    pub fn glp_ios_del_row(T: *mut glp_tree, i: c_int);
    pub fn glp_ios_clear_pool(T: *mut glp_tree);
    pub fn glp_ios_can_branch(T: *mut glp_tree, j: c_int) -> c_int;
    pub fn glp_ios_branch_upon(T: *mut glp_tree, j: c_int, sel: c_int);
    pub fn glp_ios_select_node(T: *mut glp_tree, p: c_int);
    pub fn glp_ios_heur_sol(T: *mut glp_tree, x: void) -> c_int;
    pub fn glp_ios_terminate(T: *mut glp_tree);
    pub fn glp_init_mpscp(parm: *mut Mpscp);
    pub fn glp_init_cpxcp(parm: *mut Cpxcp);
    pub fn glp_mpl_alloc_wksp() -> *mut glp_tran;
    pub fn glp_mpl_read_model(tran: *mut glp_tran, fname: *schar,
                              skip: c_int) -> c_int;
    pub fn glp_mpl_read_data(tran: *mut glp_tran, fname: *schar) -> c_int;
    pub fn glp_mpl_generate(tran: *mut glp_tran, fname: *schar) -> c_int;
    pub fn glp_mpl_build_prob(tran: *mut glp_tran, prob: *mut Prob);
    pub fn glp_mpl_postsolve(tran: *mut glp_tran, prob: *mut Prob,
                             sol: c_int) -> c_int;
    pub fn glp_mpl_free_wksp(tran: *mut glp_tran);
    pub fn glp_main(argc: c_int, argv: void) -> c_int;
    pub fn glp_init_env() -> c_int;
    pub fn glp_version() -> *schar;
    pub fn glp_free_env() -> c_int;
    pub fn glp_puts(s: *schar);
    pub fn glp_printf(fmt: *schar);
    pub fn glp_term_out(flag: c_int) -> c_int;
    pub fn glp_term_hook(func:
                             extern "C" fn(arg1: *mut void, arg2: *schar)
                                 -> c_int, info: *mut void);
    pub fn glp_open_tee(name: *schar) -> c_int;
    pub fn glp_close_tee() -> c_int;
    pub fn glp_error_(file: *schar, line: c_int) -> glp_errfunc;
    pub fn glp_assert_(expr: *schar, file: *schar, line: c_int);
    pub fn glp_error_hook(func: extern "C" fn(arg1: *mut void),
                          info: *mut void);
    pub fn glp_alloc(n: c_int, size: c_int) -> *mut void;
    pub fn glp_realloc(ptr: *mut void, n: c_int, size: c_int) ->
     *mut void;
    pub fn glp_free(ptr: *mut void);
    pub fn glp_mem_limit(limit: c_int);
    pub fn glp_mem_usage(count: *mut c_int, cpeak: *mut c_int,
                         total: *mut size_t, tpeak: *mut size_t);
    pub fn glp_create_graph(v_size: c_int, a_size: c_int) -> *mut Graph;
    pub fn glp_set_graph_name(G: *mut Graph, name: *schar);
    pub fn glp_add_vertices(G: *mut Graph, nadd: c_int) -> c_int;
    pub fn glp_set_vertex_name(G: *mut Graph, i: c_int, name: *schar);
    pub fn glp_add_arc(G: *mut Graph, i: c_int, j: c_int) -> *mut Arc;
    pub fn glp_del_vertices(G: *mut Graph, ndel: c_int, num: void);
    pub fn glp_del_arc(G: *mut Graph, a: *mut Arc);
    pub fn glp_erase_graph(G: *mut Graph, v_size: c_int, a_size: c_int);
    pub fn glp_delete_graph(G: *mut Graph);
    pub fn glp_create_v_index(G: *mut Graph);
    pub fn glp_find_vertex(G: *mut Graph, name: *schar) -> c_int;
    pub fn glp_delete_v_index(G: *mut Graph);
    pub fn glp_read_graph(G: *mut Graph, fname: *schar) -> c_int;
    pub fn glp_write_graph(G: *mut Graph, fname: *schar) -> c_int;
    pub fn glp_mincost_okalg(G: *mut Graph, v_rhs: c_int, a_low: c_int,
                             a_cap: c_int, a_cost: c_int, sol: *mut c_double,
                             a_x: c_int, v_pi: c_int) -> c_int;
    pub fn glp_mincost_relax4(G: *mut Graph, v_rhs: c_int, a_low: c_int,
                              a_cap: c_int, a_cost: c_int, crash: c_int,
                              sol: *mut c_double, a_x: c_int, a_rc: c_int) ->
     c_int;
    pub fn glp_maxflow_ffalg(G: *mut Graph, s: c_int, t: c_int,
                             a_cap: c_int, sol: *mut c_double, a_x: c_int,
                             v_cut: c_int) -> c_int;
    pub fn glp_check_asnprob(G: *mut Graph, v_set: c_int) -> c_int;

    pub fn ios_get_prob(T: *mut glp_tree) -> *mut Prob {
    }
    */
