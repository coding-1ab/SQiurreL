use crate::query::parser::{Clause, Expr, Stmt};

pub enum Plan {
    Insert {
        table: Box<str>,
        columns: Vec<Box<str>>,
        values: Vec<Expr>,
    },
    Select {
        table: Box<str>,
        columns: Vec<Box<str>>,
        filter: Option<Box<Expr>>,
        order: Option<Vec<(Box<Expr>, bool)>>,
        limit: Option<u64>,
    },
}

pub struct Planner;

impl Planner {
    pub fn plan(&self, stmt: Stmt) -> Plan {
        match stmt {
            Stmt::Insert { table, clauses } => self.plan_insert(table, clauses),
            Stmt::Select { table, clauses } => self.plan_select(table, clauses),
            _ => todo!(),
        }
    }

    fn plan_insert(&self, table: Box<str>, clauses: Vec<Clause>) -> Plan {
        let mut columns = vec![]; // 빈 값이면 모든 컬럼
        let mut values = None; // 필수
        for clause in clauses {
            match clause {
                Clause::Columns(cols) => columns = cols,
                Clause::Values(vals) => values = Some(vals),
                _ => {} // TODO: 일단 무시
            }
        }
        let values = values.expect("VALUES clause is required for INSERT");
        Plan::Insert {
            table,
            columns,
            values,
        }
    }

    fn plan_select(&self, table: Box<str>, clauses: Vec<Clause>) -> Plan {
        let mut columns = vec![]; // 빈 값이면 모든 컬럼
        let mut filter = None; // 선택적
        let mut order = None; // 선택적
        let mut limit = None; // 선택적
        for clause in clauses {
            match clause {
                Clause::Columns(cols) => columns = cols,
                Clause::Where(expr) => filter = Some(expr),
                Clause::OrderBy(cols) => order = Some(cols),
                Clause::Limit(n) => limit = Some(n),
                _ => {} // TODO: 일단 무시
            }
        }
        Plan::Select {
            table,
            columns,
            filter,
            order,
            limit,
        }
    }
}
