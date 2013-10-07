use ffi;
use prob::Prob;

#[deriving(ToStr)]
pub enum VarType {
    Bool(bool),
    Continuous(f64),
    Integer(int)
}

pub enum Bound {
    LE(f64), // <=
    GE(f64)  // >=
}

pub struct Variable {
    name:  ~str,
    vtype: VarType,
    id:    int
}

impl Variable {
    pub fn new(name: ~str, vtype: VarType) -> Variable {
        Variable {
            name:  name,
            vtype: vtype,
            id:    0
        }
    }
}

pub struct Constraint {
    name:  ~str,
    sumof: ~[(f64, @mut Variable)],
    bound: Bound
}

impl Constraint {
    pub fn new(name: ~str, sumof: ~[(f64, @mut Variable)], bound: Bound) -> Constraint {
        Constraint {
            name:  name,
            sumof: sumof,
            bound: bound
        }
    }
}

pub struct Problem {
    name:        ~str,
    constraints: ~[Constraint],
    objective:   ~[(f64, @mut Variable)]
}

impl Problem {
    pub fn new(name: ~str, constraints: ~[Constraint], objective: ~[(f64, @mut Variable)]) -> Problem {
        Problem {
            name:        name,
            constraints: constraints,
            objective:   objective
        }
    }

    pub fn solve(&mut self, maximize: bool, mip: bool) -> f64 {
        do ffi::glp_start {
            let mut lp = Prob::new();

            for c in self.constraints.iter() {
                for &(_, v) in c.sumof.iter() {
                    v.id = -1;
                }
            }

            let mut variables = ~[];
            let mut i = 0;
            for c in self.constraints.iter() {
                for &(_, v) in c.sumof.iter() {
                    if v.id == -1 {
                        i = i + 1;
                        v.id = i;
                        variables.push(v);
                    }
                }
            }

            for &(_, v) in self.objective.iter() {
                if v.id == -1 {
                    i = i + 1;
                    v.id = i;
                    variables.push(v);
                }
            }

            lp.add_rows(self.constraints.len() as i32);
            lp.add_cols(i as i32);

            // variables names and types
            for v in variables.iter() {
                lp.set_col_name(v.id as i32, v.name);
                lp.set_col_bnds(v.id as i32, ffi::GLP_FR, 0.0, 0.0);

                match v.vtype {
                    Bool(_)       => lp.set_col_kind(v.id as i32, ffi::GLP_BV),
                    Continuous(_) => lp.set_col_kind(v.id as i32, ffi::GLP_CV),
                    Integer(_)    => lp.set_col_kind(v.id as i32, ffi::GLP_IV),
                }
            }

            let mut ia = ~[ 0 ];
            let mut ja = ~[ 0 ];
            let mut ar = ~[ 0.0 ];

            // constraints
            for (i, c) in self.constraints.iter().enumerate() {
                lp.set_row_name(i as i32 + 1, c.name);

                match c.bound {
                    GE(val) => lp.set_row_bnds(i as i32 + 1, ffi::GLP_LO, val, 0.0),
                    LE(val) => {
                        lp.set_row_bnds(i as i32 + 1, ffi::GLP_UP, 0.0, val)
                    }
                }

                for &(coeff, v) in c.sumof.iter() {
                    ia.push(i as i32 + 1);
                    ja.push(v.id as i32);
                    ar.push(coeff);
                }
            }

            // objective
            for &(coeff, v) in self.objective.iter() {
                lp.set_obj_coef(v.id as i32, coeff)
            }

            if maximize {
                lp.set_obj_dir(ffi::GLP_MAX)
            }
            else {
                lp.set_obj_dir(ffi::GLP_MIN)
            }

            lp.load_matrix(ia.len() as i32 - 1, ia, ja, ar);

            lp.simplex(None);

            if mip {
                lp.intopt(None);
            }

            for v in variables.iter() {
                let val = if mip { lp.mip_col_val(v.id as i32) }
                          else   { lp.get_col_prim(v.id as i32) };
                let newtype =
                    match v.vtype {
                        Bool(_)       => {
                            println!("val: {}", val);
                            Bool(val != 0.0)
                        },
                        Continuous(_) => {
                            println!("val: {}", val);
                            Continuous(val)
                        },
                        Integer(_)    => {
                            println!("val: {}", val);
                            Integer(val as int)
                        }
                    };

                v.vtype = newtype;
            }

            lp.get_obj_val()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_small_problem_hl() {

        /*
         * Solve:
         *  maximize	0.6 x + 0.5 y
         *  subject to	x     + 2 y <= 1
         *              3 x   + y   <= 2
         */

        let x = @mut Variable::new(~"x", Continuous(0.0));
        let y = @mut Variable::new(~"y", Continuous(0.0));

        let c1 = Constraint::new(~"p", ~[(1.0, x), (2.0, y)], LE(1.0));
        let c2 = Constraint::new(~"q", ~[(3.0, x), (1.0, y)], LE(2.0));

        let mut problem = Problem::new(~"Simple problem.", ~[c1, c2], ~[(0.6, x), (0.5, y)]);

        let solution = problem.solve(true, false);

        println!("z = { }; x = { }; y = { }", solution, x.vtype.to_str(), y.vtype.to_str());
    }
}
