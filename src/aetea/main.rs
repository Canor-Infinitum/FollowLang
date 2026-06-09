//! # Demo binary
//!
//! Shows how to use the macros and the simple compiler.

use follow_suite::{
    follower_lang::FollowerProgram, ether_ir::EtherIR,
};

fn main() {
    // 1️⃣ Parse each language using the compile‑time macros.
    let flow_expr   = flow!("{ +1_2, -i_3 }");
    println!("Parsed Flow: {:#?}", flow_expr);

    let action_expr = action!("(7m + (4n * 4o)) == 0 (mod 3)");
    println!("Parsed Action: {:#?}", action_expr);

    let maneuver_expr = maneuver!(
        "FlowGroup -> [ +1_2, -i_3 ] | ActionGroup -> [(7m ...)]"
    );
    println!("Parsed Maneuver: {:#?}", maneuver_expr);

    let ui_expr   = ui!("Schema { flow -> FlowExpr, action -> ActionExpr }");
    println!("Parsed UI: {:#?}", ui_expr);

    let elite_expr = elite!("mov FlowExpr; jmp ActionExpr");
    println!("Parsed Elite: {:#?}", elite_expr);

    // 2️⃣ Build a Follower program.
    let mut prog = FollowerProgram::new();
    prog.add_flow(flow_expr);
    prog.add_action(action_expr);
    prog.add_maneuver(maneuver_expr);
    prog.add_ui(ui_expr);
    prog.add_elite(elite_expr);

    println!("Follower Program: {:#?}", prog);

    // 3️⃣ Convert to the Ether IR.
    let ether = EtherIR::from(prog);
    println!("Ether IR: {:#?}", ether);
}