/// Macro for embedding a Flow expression at compile time.
/// ```rust
/// let e = flow!("{ +1_2, -i_3 }");
/// ```
#[macro_export]
macro_rules! flow {
    ($expr:literal) => {{
        $crate::flow_lang::FlowExpr::parse($expr)
            .expect("Failed to parse Flow expression")
    }};
}

/// Macro for embedding an Action expression at compile time.
#[macro_export]
macro_rules! action {
    ($expr:literal) => {{
        $crate::action_lang::ActionExpr::parse($expr)
            .expect("Failed to parse Action expression")
    }};
}

/// Macro for embedding a Maneuver expression at compile time.
#[macro_export]
macro_rules! maneuver {
    ($expr:literal) => {{
        $crate::maneuvers_lang::ManeuverExpr::parse($expr)
            .expect("Failed to parse Maneuver expression")
    }};
}

/// Macro for embedding a UI (United Interchange) expression at compile time.
#[macro_export]
macro_rules! ui {
    ($expr:literal) => {{
        $crate::ui_lang::UIExpr::parse($expr)
            .expect("Failed to parse UI expression")
    }};
}

/// Macro for embedding an Elite (assembly‑style) expression at compile time.
#[macro_export]
macro_rules! elite {
    ($expr:literal) => {{
        $crate::elite_lang::EliteExpr::parse($expr)
            .expect("Failed to parse Elite expression")
    }};
}