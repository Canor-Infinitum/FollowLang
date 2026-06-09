// follow-body/src/fwb.rs

#[derive(Debug, Clone, PartialEq)]
pub struct FollowPackageBody {
    pub packages: Vec<PackageSpec>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PackageSpec {
    pub name: String,
    pub headers: Vec<String>,
    pub dependencies: Vec<String>,
}

impl FollowPackageBody {
    pub fn qgct_default() -> Self {
        Self {
            packages: vec![
                PackageSpec {
                    name: "core.tensor".into(),
                    headers: vec!["tensor.h".into(), "tensorial.h".into()],
                    dependencies: vec![],
                },
                PackageSpec {
                    name: "core.expression".into(),
                    headers: vec!["expression.h".into()],
                    dependencies: vec!["core.tensor".into()],
                },
                PackageSpec {
                    name: "physics.qgct".into(),
                    headers: vec![
                        "force.h".into(),
                        "change.h".into(),
                        "composition.h".into(),
                    ],
                    dependencies: vec![
                        "core.tensor".into(),
                        "core.expression".into(),
                    ],
                },
            ],
        }
    }
}