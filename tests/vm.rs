use tradelang::bytecode::{Constant, Instruction, LocalInfo, OpCode, Program};
use tradelang::compiler::CompiledProgram;
use tradelang::diagnostic::RuntimeError;
use tradelang::runtime::{run, Bar, VmLimits};
use tradelang::types::{SlotKind, Type, Value};

fn empty_locals() -> Vec<LocalInfo> {
    vec![
        LocalInfo {
            name: Some("open".into()),
            ty: Type::SeriesF64,
            kind: SlotKind::Series,
            hidden: false,
        },
        LocalInfo {
            name: Some("high".into()),
            ty: Type::SeriesF64,
            kind: SlotKind::Series,
            hidden: false,
        },
        LocalInfo {
            name: Some("low".into()),
            ty: Type::SeriesF64,
            kind: SlotKind::Series,
            hidden: false,
        },
        LocalInfo {
            name: Some("close".into()),
            ty: Type::SeriesF64,
            kind: SlotKind::Series,
            hidden: false,
        },
        LocalInfo {
            name: Some("volume".into()),
            ty: Type::SeriesF64,
            kind: SlotKind::Series,
            hidden: false,
        },
        LocalInfo {
            name: Some("time".into()),
            ty: Type::SeriesF64,
            kind: SlotKind::Series,
            hidden: false,
        },
    ]
}

fn bars() -> Vec<Bar> {
    vec![Bar {
        open: 1.0,
        high: 2.0,
        low: 0.5,
        close: 1.5,
        volume: 10.0,
        time: 1_700_000_000_000.0,
    }]
}

#[test]
fn tiny_program_push_add_plot_executes() {
    let program = Program {
        instructions: vec![
            Instruction::new(OpCode::LoadConst).with_a(0),
            Instruction::new(OpCode::LoadConst).with_a(1),
            Instruction::new(OpCode::Add),
            Instruction::new(OpCode::CallBuiltin).with_a(9).with_b(1),
            Instruction::new(OpCode::Return),
        ],
        constants: vec![
            Constant::Value(Value::F64(1.0)),
            Constant::Value(Value::F64(2.0)),
        ],
        locals: empty_locals(),
        history_capacity: 2,
        plot_count: 1,
    };
    let compiled = CompiledProgram {
        program,
        source: String::new(),
    };
    let outputs = run(&compiled, &bars(), VmLimits::default()).expect("vm should run");
    assert_eq!(outputs.plots[0].points[0].value, Some(3.0));
}

#[test]
fn stack_underflow_is_reported() {
    let program = Program {
        instructions: vec![
            Instruction::new(OpCode::Add),
            Instruction::new(OpCode::Return),
        ],
        constants: vec![],
        locals: empty_locals(),
        history_capacity: 2,
        plot_count: 0,
    };
    let compiled = CompiledProgram {
        program,
        source: String::new(),
    };
    let err = run(&compiled, &bars(), VmLimits::default()).expect_err("expected stack underflow");
    assert!(matches!(err, RuntimeError::StackUnderflow { .. }));
}

#[test]
fn invalid_jump_is_reported() {
    let program = Program {
        instructions: vec![
            Instruction::new(OpCode::Jump).with_a(999),
            Instruction::new(OpCode::Return),
        ],
        constants: vec![],
        locals: empty_locals(),
        history_capacity: 2,
        plot_count: 0,
    };
    let compiled = CompiledProgram {
        program,
        source: String::new(),
    };
    let err = run(&compiled, &bars(), VmLimits::default()).expect_err("expected invalid jump");
    assert!(matches!(err, RuntimeError::InvalidJump { .. }));
}

#[test]
fn instruction_budget_exhaustion_is_reported() {
    let compiled = tradelang::compile("plot(sma(close, 5))").expect("script should compile");
    let fixture = vec![
        Bar {
            open: 1.0,
            high: 1.0,
            low: 1.0,
            close: 1.0,
            volume: 1.0,
            time: 1.0,
        };
        6
    ];
    let err = run(
        &compiled,
        &fixture,
        VmLimits {
            max_instructions_per_bar: 3,
            max_history_capacity: 32,
        },
    )
    .expect_err("budget should exhaust");
    assert!(matches!(
        err,
        RuntimeError::InstructionBudgetExceeded { .. }
    ));
}
