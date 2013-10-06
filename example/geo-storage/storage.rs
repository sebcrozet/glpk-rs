extern mod glpkrs;
use glpkrs::hl::*;
use std::io;
use glpkrs::hl::Variable;
use glpkrs::hl::Constraint;
use glpkrs::hl::Problem;

pub struct Site {
    x: f64,
    y: f64,
    cap: f64
}

impl Site {
    pub fn new(x: f64, y: f64, cap: f64) -> Site {
        Site {
            x: x,
            y: y,
            cap: cap
        }
    }

    pub fn new_from_line(line: &str) -> Site {
        let mut iter = line.split_iter(' ');

        let x          = iter.next().unwrap();
        let y          = iter.next().unwrap();
        let cap        = iter.next().unwrap();

        Site::new(
            parse_f64(x),
            parse_f64(y),
            parse_f64(cap)
            )
    }
}

fn parse_f64(i: &str) -> f64 {
    FromStr::from_str(i).unwrap()
}

fn parse_uint(i: &str) -> uint {
    FromStr::from_str(i).unwrap()
}

fn solve_geo_storage(path: &str) {


    let lines   = io::read_whole_file_str(&PosixPath(path)).expect("Unable to read file: " + path);

    let mut sel_sites: ~[@mut Variable] = ~[];
    let mut sites: ~[Site] = ~[];
    let mut pb_vec: ~[(f64, @mut Variable)] = ~[];

    let mut lines_i = lines.any_line_iter();

    let mut first_line_iter = lines_i.next().unwrap().split_iter(' ');
    first_line_iter.next();
    let m = parse_uint(first_line_iter.next().unwrap());

    let mut n = 0u;
    for l in lines_i {
        if n < m {
            sel_sites.push(@mut Variable::new("b" + n.to_str(), Bool(true)));
        }
        else {
            sel_sites.push(@mut Variable::new("b" + n.to_str(), Bool(false)));
        }
        sites.push(Site::new_from_line(l));
        pb_vec.push((sites[n].cap, sel_sites[n]));
        n = n + 1;
    }

    let mut constraints: ~[Constraint] = ~[];
    let mut cst_vec: ~[(f64, @mut Variable)] = ~[];

    for s in sel_sites.iter() {
        cst_vec.push((1.0, *s));
    }

    constraints.push(Constraint::new(~"less", cst_vec.clone(), LE(m as f64)));
    constraints.push(Constraint::new(~"great", cst_vec.clone(), GE(m as f64)));

    for (i, si) in sites.iter().enumerate() {
        for (j, sj) in sites.iter().enumerate() {
            let diffx = (si.x - sj.x);
            let diffy = (si.y - sj.y);
            let dist = diffx * diffx + diffy * diffy;
            if dist >= 2500.0 {
                constraints.push(Constraint::new("c" + i.to_str() + j.to_str(),
                                                 ~[(1.0, sel_sites[i]), (1.0, sel_sites[j])],
                                                 LE(1.0)
                                                 ));
            }
        }
    }

    let mut problem = Problem::new(~"Geographic", constraints, pb_vec);
    let solution = problem.solve(true);
    for (i, s) in sel_sites.iter().enumerate() {
        println(s.vtype.to_str());
    }
}

pub fn main()
{
    solve_geo_storage("./example/geo-storage/input.txt");
}
