#![cfg(feature = "component")]
cargo_component_bindings::generate!();
use std::cmp::Ordering;

use self::bindings::exports::jcbhmr::semver::semver::{
    Comparator, Guest, GuestBuildMetadata, GuestPrerelease, Op, OwnBuildMetadata, OwnError,
    OwnPrerelease, Version, VersionReq,
};
use semver_upstream;

pub struct Component;
impl Guest for Component {
    fn comparator_parse(text: String) -> Result<Comparator, OwnError> {
        semver_upstream::Comparator::parse(text.as_str()).map_or_else(
            |e| Err(OwnError::new(Error(e))),
            |v| {
                Ok(Comparator {
                    op: match v.op {
                        semver_upstream::Op::Exact => Op::Exact,
                        semver_upstream::Op::Greater => Op::Greater,
                        semver_upstream::Op::GreaterEq => Op::GreaterEq,
                        semver_upstream::Op::Less => Op::Less,
                        semver_upstream::Op::LessEq => Op::LessEq,
                        semver_upstream::Op::Tilde => Op::Tilde,
                        semver_upstream::Op::Caret => Op::Caret,
                        semver_upstream::Op::Wildcard => Op::Wildcard,
                        _ => todo!(),
                    },
                    major: v.major,
                    minor: v.minor,
                    patch: v.patch,
                    pre: OwnPrerelease::new(Prerelease(v.pre)),
                })
            },
        )
    }
    fn comparator_matches(self_: Comparator, version: Version) -> bool {
        semver_upstream::Comparator {
            op: match self_.op {
                Op::Exact => semver_upstream::Op::Exact,
                Op::Greater => semver_upstream::Op::Greater,
                Op::GreaterEq => semver_upstream::Op::GreaterEq,
                Op::Less => semver_upstream::Op::Less,
                Op::LessEq => semver_upstream::Op::LessEq,
                Op::Tilde => semver_upstream::Op::Tilde,
                Op::Caret => semver_upstream::Op::Caret,
                Op::Wildcard => semver_upstream::Op::Wildcard,
            },
            major: self_.major,
            minor: self_.minor,
            patch: self_.patch,
            pre: self_.pre.0.clone(),
        }
        .matches(&semver_upstream::Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            pre: version.pre.0.clone(),
            build: version.build.0.clone(),
        })
    }

    fn version_new(major: u64, minor: u64, patch: u64) -> Version {
        let self_ = semver_upstream::Version::new(major, minor, patch);
        Version {
            major: self_.major,
            minor: self_.minor,
            patch: self_.patch,
            pre: OwnPrerelease::new(Prerelease(self_.pre)),
            build: OwnBuildMetadata::new(BuildMetadata(self_.build)),
        }
    }
    fn version_parse(
        text: cargo_component_bindings::rt::string::String,
    ) -> Result<Version, OwnError> {
        semver_upstream::Version::parse(text.as_str()).map_or_else(
            |e| Err(OwnError::new(Error(e))),
            |v| {
                Ok(Version {
                    major: v.major,
                    minor: v.minor,
                    patch: v.patch,
                    pre: OwnPrerelease::new(Prerelease(v.pre)),
                    build: OwnBuildMetadata::new(BuildMetadata(v.build)),
                })
            },
        )
    }
    fn version_cmp_precedence(a: Version, b: Version) -> i8 {
        let v = semver_upstream::Version::cmp_precedence(
            &semver_upstream::Version {
                major: a.major,
                minor: a.minor,
                patch: a.patch,
                pre: a.pre.0.clone(),
                build: a.build.0.clone(),
            },
            &semver_upstream::Version {
                major: b.major,
                minor: b.minor,
                patch: b.patch,
                pre: b.pre.0.clone(),
                build: b.build.0.clone(),
            },
        );
        match v {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    fn version_req_star() -> VersionReq {
        VersionReq {
            comparators: semver_upstream::VersionReq::STAR
                .comparators
                .iter()
                .map(|c| Comparator {
                    op: match c.op {
                        semver_upstream::Op::Exact => Op::Exact,
                        semver_upstream::Op::Greater => Op::Greater,
                        semver_upstream::Op::GreaterEq => Op::GreaterEq,
                        semver_upstream::Op::Less => Op::Less,
                        semver_upstream::Op::LessEq => Op::LessEq,
                        semver_upstream::Op::Tilde => Op::Tilde,
                        semver_upstream::Op::Caret => Op::Caret,
                        semver_upstream::Op::Wildcard => Op::Wildcard,
                        _ => todo!(),
                    },
                    major: c.major,
                    minor: c.minor,
                    patch: c.patch,
                    pre: OwnPrerelease::new(Prerelease(c.pre.clone())),
                })
                .collect(),
        }
    }
    fn version_req_parse(text: String) -> Result<VersionReq, OwnError> {
        semver_upstream::VersionReq::parse(text.as_str()).map_or_else(
            |e| Err(OwnError::new(Error(e))),
            |v| {
                Ok(VersionReq {
                    comparators: v
                        .comparators
                        .iter()
                        .map(|c| Comparator {
                            op: match c.op {
                                semver_upstream::Op::Exact => Op::Exact,
                                semver_upstream::Op::Greater => Op::Greater,
                                semver_upstream::Op::GreaterEq => Op::GreaterEq,
                                semver_upstream::Op::Less => Op::Less,
                                semver_upstream::Op::LessEq => Op::LessEq,
                                semver_upstream::Op::Tilde => Op::Tilde,
                                semver_upstream::Op::Caret => Op::Caret,
                                semver_upstream::Op::Wildcard => Op::Wildcard,
                                _ => todo!(),
                            },
                            major: c.major,
                            minor: c.minor,
                            patch: c.patch,
                            pre: OwnPrerelease::new(Prerelease(c.pre.clone())),
                        })
                        .collect(),
                })
            },
        )
    }
    fn version_req_matches(self_: VersionReq, version: Version) -> bool {
        semver_upstream::VersionReq {
            comparators: self_
                .comparators
                .iter()
                .map(|c| semver_upstream::Comparator {
                    op: match c.op {
                        Op::Exact => semver_upstream::Op::Exact,
                        Op::Greater => semver_upstream::Op::Greater,
                        Op::GreaterEq => semver_upstream::Op::GreaterEq,
                        Op::Less => semver_upstream::Op::Less,
                        Op::LessEq => semver_upstream::Op::LessEq,
                        Op::Tilde => semver_upstream::Op::Tilde,
                        Op::Caret => semver_upstream::Op::Caret,
                        Op::Wildcard => semver_upstream::Op::Wildcard,
                    },
                    major: c.major,
                    minor: c.minor,
                    patch: c.patch,
                    pre: c.pre.0.clone(),
                })
                .collect(),
        }
        .matches(&semver_upstream::Version {
            major: version.major,
            minor: version.minor,
            patch: version.patch,
            pre: version.pre.0.clone(),
            build: version.build.0.clone(),
        })
    }
}

pub struct BuildMetadata(semver_upstream::BuildMetadata);
impl GuestBuildMetadata for BuildMetadata {
    fn empty() -> OwnBuildMetadata {
        OwnBuildMetadata::new(BuildMetadata(semver_upstream::BuildMetadata::EMPTY))
    }
    fn new(text: String) -> Result<OwnBuildMetadata, OwnError> {
        semver_upstream::BuildMetadata::new(text.as_str()).map_or_else(
            |e| Err(OwnError::new(Error(e))),
            |v| Ok(OwnBuildMetadata::new(BuildMetadata(v))),
        )
    }

    fn as_str(&self) -> String {
        self.0.as_str().to_string()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub struct Error(semver_upstream::Error);

pub struct Prerelease(semver_upstream::Prerelease);
impl GuestPrerelease for Prerelease {
    fn empty() -> OwnPrerelease {
        OwnPrerelease::new(Prerelease(semver_upstream::Prerelease::EMPTY))
    }
    fn new(text: String) -> Result<OwnPrerelease, OwnError> {
        semver_upstream::Prerelease::new(text.as_str()).map_or_else(
            |e| Err(OwnError::new(Error(e))),
            |v| Ok(OwnPrerelease::new(Prerelease(v))),
        )
    }

    fn as_str(&self) -> String {
        self.0.as_str().to_string()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
