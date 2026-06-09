use follow_lang::{
    action::ActionSymbolSpec,
    elite::EliteSchemaSpec,
    ether::EpsilonSpec,
    flow::FlowScaleSpec,
    follower::{Backend, FollowHeaderSpec, FollowProjectSpec, MetricTensor6Spec},
    maneuvers::ManeuverGroupSpec,
    registry::{SuiteRegistry, SuiteSpec},
    united::UnitedInterchangeSpec,
};

fn main() {
    println!("\n--- FOLLOW SUITE PHASE A SPEC DEMO ---");

    let flow_scale = FlowScaleSpec::fundamental();
    println!("Flow r_s/l_p: {}", flow_scale.r_s_over_l_p);
    println!("Flow r_s: {}", flow_scale.r_s);

    let action_symbol = ActionSymbolSpec::arbitrary_f();
    println!("Action equation: {}", action_symbol.equation);

    let maneuver_group = ManeuverGroupSpec::new(flow_scale.clone(), action_symbol.clone());
    println!("Maneuver Δ(A): {}", maneuver_group.delta_a);

    let united = UnitedInterchangeSpec::new(maneuver_group.clone());
    println!("United extraction: {}", united.runtime_extraction);

    let elite = EliteSchemaSpec::new(united.clone());
    println!("Elite radius formula: {}", elite.radius_formula);

    let project = FollowProjectSpec {
        backend: Backend::Rust,
        metric_tensor: MetricTensor6Spec::default_follow_metric(),
        header: FollowHeaderSpec::default_header(),
    };

    println!("Follower metric tensor: {}", project.metric_tensor.metric);
    println!("Follower header relation: {}", project.header.relation);

    let epsilon = EpsilonSpec::default_spec();
    println!("Ether epsilon: {}", epsilon.eps);

    let mut suite_registry = SuiteRegistry::new();
    suite_registry.insert("flow_scale", SuiteSpec::FlowScale(flow_scale)).unwrap();
    suite_registry.insert("action_symbol", SuiteSpec::ActionSymbol(action_symbol)).unwrap();
    suite_registry.insert("maneuver_group", SuiteSpec::ManeuverGroup(maneuver_group)).unwrap();
    suite_registry.insert("united", SuiteSpec::UnitedInterchange(united)).unwrap();
    suite_registry.insert("elite", SuiteSpec::EliteSchema(elite)).unwrap();
    suite_registry.insert("project", SuiteSpec::FollowProject(project)).unwrap();
    suite_registry.insert("epsilon", SuiteSpec::Epsilon(epsilon)).unwrap();

    println!("Suite registry keys: {:?}", suite_registry.keys());
}
