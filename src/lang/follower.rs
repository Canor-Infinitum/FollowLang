use crate::core::formula::Formula;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Backend {
	// Supported Languages
    Rust,
    C,
    Cpp,
    Python,
    Prolog,
    Delphi,
	// Interoperability
	Flow,
	Action,
	Maneuvers,
	United,
	Elite,
	Ether,
	// Interchange
	JSON,
	XML,
	// ...
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetricTensor6Spec {
    pub coordinates: Vec<String>,
    pub metric: Formula,
}

impl MetricTensor6Spec {
    pub fn default_follow_metric() -> Self {
        // Preserving user's coordinate list verbatim
        let coordinates = vec![
            "t".into(),
            "r".into(),
            "v".into(),
            "p".into(),
            "a".into(),
            "u".into(),
            "v".into(),
        ];

        let metric = Formula::Text(
            "g_(alpha beta)^6 := [(-c^2,0,0,0,0,0);(0,1,0,0,0,0);(0,0,r^2,0,0,0);(0,0,0,r^2*sin(v),0,0);(0,0,0,0,g[v],0);(0,0,0,0,0,g[v])] * f[t,r,v,p]"
                .into(),
        );

        Self { coordinates, metric }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FollowHeaderSpec {
    pub n_symbol: String,
    pub p_symbol: String,
    pub relation: Formula,
}

impl FollowHeaderSpec {
    pub fn default_header() -> Self {
        let relation = Formula::Text(
            "N * p = (1 / r_s) * (c^3 / (h_bar * G))^(-1) = (l_p^2) / r_s".into(),
        );

        Self {
            n_symbol: "N".into(),
            p_symbol: "p".into(),
            relation,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FollowProjectSpec {
    pub backend: Backend,
    pub metric_tensor: MetricTensor6Spec,
    pub header: FollowHeaderSpec,
}

