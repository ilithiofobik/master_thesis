use crate::graphs::Graph;
use good_lp::*;

pub fn schnyder_mps(g: &Graph) {
    let n = g.num_of_vertices();
    let m = g.num_of_edges();

    let mut lp = LinearProgram::new("SchnyderMPS");

    let mut x = vec![];
    for i in 0..n {
        x.push(lp.add_var("x", 0.0, 1.0));
    }

    let mut y = vec![];
    for i in 0..m {
        y.push(lp.add_var("y", 0.0, 1.0));
    }

    for i in 0..n {
        let mut expr = LpExpression::from(x[i]);
        for j in 0..m {
            if g.edge(i, j) {
                expr = expr + LpExpression::from(y[j]);
            }
        }
        lp.add_constraint(expr, GREATER_EQ, 1.0);
    }

    for j in 0..m {
        let mut expr = LpExpression::from(y[j]);
        for i in 0..n {
            if g.edge(i, j) {
                expr = expr + LpExpression::from(x[i]);
            }
        }
        lp.add_constraint(expr, LESS_EQ, 1.0);
    }

    lp.set_objective(
        LpObjective::Maximize,
        LpExpression::from(x.iter().sum::<LpExpression>() + y.iter().sum::<LpExpression>()),
    );

    let solution = lp.solve().unwrap();

    for i in 0..n {
        println!("x[{}] = {}", i, solution[x[i]]);
    }

    for j in 0..m {
        println!("y[{}] = {}", j, solution[y[j]]);
    }
}
