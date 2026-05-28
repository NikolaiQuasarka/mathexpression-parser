use crate::parser::{BinaryOp, Expr};

pub fn create_data_set() -> Vec<(&'static str, Expr, f64)> {
    vec![
        (
            "3+2",
            Expr::Binary {
                left: Box::new(Expr::Number(3.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            },
            5.0,
        ),
        (
            "7-3+2",
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(7.0)),
                    op: BinaryOp::Sub,
                    right: Box::new(Expr::Number(3.0)),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(2.0)),
            },
            6.0,
        ),
        (
            "4*5+6/2",
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(4.0)),
                    op: BinaryOp::Mul,
                    right: Box::new(Expr::Number(5.0)),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(6.0)),
                    op: BinaryOp::Div,
                    right: Box::new(Expr::Number(2.0)),
                }),
            },
            23.0,
        ),
        (
            "10+5-3*2/4",
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(10.0)),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(5.0)),
                        op: BinaryOp::Sub,
                        right: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(3.0)),
                                op: BinaryOp::Mul,
                                right: Box::new(Expr::Number(2.0)),
                            }),
                            op: BinaryOp::Div,
                            right: Box::new(Expr::Number(4.0)),
                        }),
                    }),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(0.0)),
            },
            13.5,
        ),
        (
            "8*3+2-5/1+4",
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(8.0)),
                                op: BinaryOp::Mul,
                                right: Box::new(Expr::Number(3.0)),
                            }),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Number(2.0)),
                        }),
                        op: BinaryOp::Sub,
                        right: Box::new(Expr::Binary {
                            left: Box::new(Expr::Number(5.0)),
                            op: BinaryOp::Div,
                            right: Box::new(Expr::Number(1.0)),
                        }),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(4.0)),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(0.0)),
            },
            25.0,
        ),
        (
            "2+3*4-5/6+7*8-9",
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(2.0)),
                                    op: BinaryOp::Add,
                                    right: Box::new(Expr::Binary {
                                        left: Box::new(Expr::Number(3.0)),
                                        op: BinaryOp::Mul,
                                        right: Box::new(Expr::Number(4.0)),
                                    }),
                                }),
                                op: BinaryOp::Sub,
                                right: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(5.0)),
                                    op: BinaryOp::Div,
                                    right: Box::new(Expr::Number(6.0)),
                                }),
                            }),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(7.0)),
                                op: BinaryOp::Mul,
                                right: Box::new(Expr::Number(8.0)),
                            }),
                        }),
                        op: BinaryOp::Sub,
                        right: Box::new(Expr::Number(9.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(0.0)),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(0.0)),
            },
            60.16666666666667,
        ),
        (
            "1+2*3+4*5+6*7+8*9",
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(1.0)),
                                op: BinaryOp::Add,
                                right: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(2.0)),
                                    op: BinaryOp::Mul,
                                    right: Box::new(Expr::Number(3.0)),
                                }),
                            }),
                            op: BinaryOp::Add,
                            right: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(4.0)),
                                op: BinaryOp::Mul,
                                right: Box::new(Expr::Number(5.0)),
                            }),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Binary {
                            left: Box::new(Expr::Number(6.0)),
                            op: BinaryOp::Mul,
                            right: Box::new(Expr::Number(7.0)),
                        }),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(8.0)),
                        op: BinaryOp::Mul,
                        right: Box::new(Expr::Number(9.0)),
                    }),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(0.0)),
            },
            141.0,
        ),
        (
            "10-2*3+4/2-5+6*7-8/4+9",
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Binary {
                                left: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Binary {
                                        left: Box::new(Expr::Binary {
                                            left: Box::new(Expr::Number(10.0)),
                                            op: BinaryOp::Sub,
                                            right: Box::new(Expr::Binary {
                                                left: Box::new(Expr::Number(2.0)),
                                                op: BinaryOp::Mul,
                                                right: Box::new(Expr::Number(3.0)),
                                            }),
                                        }),
                                        op: BinaryOp::Add,
                                        right: Box::new(Expr::Binary {
                                            left: Box::new(Expr::Number(4.0)),
                                            op: BinaryOp::Div,
                                            right: Box::new(Expr::Number(2.0)),
                                        }),
                                    }),
                                    op: BinaryOp::Sub,
                                    right: Box::new(Expr::Number(5.0)),
                                }),
                                op: BinaryOp::Add,
                                right: Box::new(Expr::Binary {
                                    left: Box::new(Expr::Number(6.0)),
                                    op: BinaryOp::Mul,
                                    right: Box::new(Expr::Number(7.0)),
                                }),
                            }),
                            op: BinaryOp::Sub,
                            right: Box::new(Expr::Binary {
                                left: Box::new(Expr::Number(8.0)),
                                op: BinaryOp::Div,
                                right: Box::new(Expr::Number(4.0)),
                            }),
                        }),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Number(9.0)),
                    }),
                    op: BinaryOp::Add,
                    right: Box::new(Expr::Number(0.0)),
                }),
                op: BinaryOp::Add,
                right: Box::new(Expr::Number(0.0)),
            },
            50.0,
        ),
        (
            "2^3",
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                op: BinaryOp::Pow,
                right: Box::new(Expr::Number(3.0)),
            },
            8.0,
        ),
        (
            "4+5^9*3-3^2",
            Expr::Binary {
                left: Box::new(Expr::Number(4.0)),
                op: BinaryOp::Add,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Binary {
                        left: Box::new(Expr::Binary {
                            left: Box::new(Expr::Number(5.0)),
                            op: BinaryOp::Pow,
                            right: Box::new(Expr::Number(9.0)),
                        }),
                        op: BinaryOp::Mul,
                        right: Box::new(Expr::Number(3.0)),
                    }),
                    op: BinaryOp::Sub,
                    right: Box::new(Expr::Binary {
                        left: Box::new(Expr::Number(3.0)),
                        op: BinaryOp::Pow,
                        right: Box::new(Expr::Number(2.0)),
                    }),
                }),
            },
            5_859_370.0,
        ),
    ]
}
