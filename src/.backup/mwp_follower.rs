use follow_core::core::{
    int::Int,
    number::Number,
    observable::Observable,
    operator::{Op, Operator},
    symbolic::{Reference, Scalar, SymbolicValue, Value},
    token::Token,
};

use follow_lang::{
    action::ActionNode,
    ether::{EtherIR, EtherNode},
    flow::FlowNode,
    maneuvers::ManeuverNode,
};

use follow_syntax::{
    ast::{
        Atom, AtomCore, AtomHead, Definition, Element, Expr, Literal, Program, Sigil, SigilNs,
        TopLevel,
    },
    semantic::SemanticNode,
};

fn main() {
    // ------------------------------------------------------------
    // 1. Core evaluation path
    // ------------------------------------------------------------
    let v = Value {
        pre: Scalar::Text("Follow".to_string()),
        peri: Scalar::Number(Number::from_usize(1)),
        post: Scalar::Int(Int::exact(2)),
    };

    let r = Reference {
        pre: "pre".into(),
        peri: "peri".into(),
        post: "post".into(),
    };

    println!("VALUE.eval() -> {:?}", v.eval());
    println!("REFERENCE.eval() -> {:?}", r.eval());

    let obs_lhs = Observable {
        pre: SymbolicValue::Value(v.clone()),
        peri: SymbolicValue::Value(v.clone()),
        post: SymbolicValue::Value(v.clone()),
        field_pre: Scalar::Text("lhs".into()),
        field_peri: Scalar::Text("lhs_peri".into()),
        field_post: Scalar::Text("lhs_post".into()),
    };

    let obs_op = Observable {
        pre: SymbolicValue::Reference(r.clone()),
        peri: SymbolicValue::Reference(r.clone()),
        post: SymbolicValue::Reference(r.clone()),
        field_pre: Scalar::Text("operator_pre".into()),
        field_peri: Scalar::Text("and".into()),
        field_post: Scalar::Text("operator_post".into()),
    };

    let obs_rhs = Observable {
        pre: SymbolicValue::Value(v.clone()),
        peri: SymbolicValue::Value(v.clone()),
        post: SymbolicValue::Value(v.clone()),
        field_pre: Scalar::Text("rhs".into()),
        field_peri: Scalar::Text("rhs_peri".into()),
        field_post: Scalar::Text("rhs_post".into()),
    };

    let op = Operator {
        lhs: obs_lhs,
        op: obs_op,
        rhs: obs_rhs,
    };

    println!("OP.eval() -> {:?}", op.eval());

    // ------------------------------------------------------------
    // 2. Syntax AST path (hand-constructed, parser comes later)
    // ------------------------------------------------------------
    let flow_atom = Atom {
        core: AtomCore {
            head: AtomHead::Path(vec![SigilNs {
                sigil: Sigil::Type,
                head: Some("flow".into()),
                path: vec![],
            }]),
            invocations: vec![],
            templates: vec![],
        },
        definition: Some(Definition::Colon(Expr::Element(Element::Literal(
            Literal::String("RBRBRBRBRBRBRBRR".into()),
        )))),
    };

    let program = Program {
        items: vec![TopLevel::Atom(flow_atom.clone())],
    };

    println!("PROGRAM -> {:?}", program);

    // ------------------------------------------------------------
    // 3. Semantic lowering path
    // ------------------------------------------------------------
    let flow_semantic = SemanticNode::from_atom_core(
        &flow_atom.core,
        flow_atom.definition.as_ref(),
    )
    .classify();

    let action_semantic = SemanticNode {
        domain: follow_syntax::semantic::Domain::Action,
        sigil: Some(Sigil::Type),
        identifier: "action".into(),
        invocations: vec![],
        templates: 0,
        definition_kind: Some(":"),
        source_repr: "4*m * 7*n = 4*o (mod X)".into(),
    };

    let maneuver_semantic = SemanticNode {
        domain: follow_syntax::semantic::Domain::Maneuvers,
        sigil: Some(Sigil::Type),
        identifier: "maneuver".into(),
        invocations: vec![],
        templates: 0,
        definition_kind: Some(":"),
        source_repr: "(&flow and &action)".into(),
    };

    // ------------------------------------------------------------
    // 4. Sub-language node path
    // ------------------------------------------------------------
    let flow = FlowNode {
        name: "flow".into(),
        pattern: "RBRBRBRBRBRBRBRR".into(),
        semantic: flow_semantic,
    };

    let action = ActionNode {
        name: "action".into(),
        relation: "4*m * 7*n = 4*o (mod X)".into(),
        semantic: action_semantic,
    };

    let maneuver = ManeuverNode {
        name: "maneuver".into(),
        composition: format!("({} and {})", flow.pattern, action.normalize()),
        weight: Some(1.0),
        semantic: maneuver_semantic,
    };

    println!("FLOW.expand(8) -> {}", flow.expand(8));
    println!("ACTION.normalize() -> {}", action.normalize());
    println!("MANEUVER.eval() -> {}", maneuver.eval());

    // ------------------------------------------------------------
    // 5. Ether IR lowering path
    // ------------------------------------------------------------
    let mut ir = EtherIR::default();
    ir.link("std.follow");
    ir.push_node(EtherNode::Symbol(flow.name.clone()));
    ir.push_node(EtherNode::Constant(action.relation.clone()));
    ir.push_node(EtherNode::Compose {
        lhs: Box::new(EtherNode::Symbol("flow".into())),
        op: "and".into(),
        rhs: Box::new(EtherNode::Symbol("action".into())),
    });

    println!("ETHER IR -> {:#?}", ir);
}