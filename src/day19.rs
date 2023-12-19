use std::collections::HashMap;

use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PartComponent {
    X,
    M,
    A,
    S,
}

impl PartComponent {
    fn from_str(c: &str) -> Self {
        match c {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Invalid component str {}", c),
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    dest: &'a str,
}

#[derive(Debug)]
struct Condition {
    component: PartComponent,
    gt: bool,
    comp_target: u32,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get_component(&self, c: &PartComponent) -> u32 {
        match c {
            PartComponent::X => self.x,
            PartComponent::M => self.m,
            PartComponent::A => self.a,
            PartComponent::S => self.s,
        }
    }

    fn rating_sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Copy, Debug)]
struct ComponentRange {
    start: u32,
    end: u32,
}

impl ComponentRange {
    fn new() -> Self {
        Self {
            start: 1,
            end: 4000,
        }
    }
}

#[derive(Clone, Debug)]
struct PartRange<'a> {
    x: ComponentRange,
    m: ComponentRange,
    a: ComponentRange,
    s: ComponentRange,
    workflow: &'a str,
}

impl<'a> PartRange<'a> {
    fn new() -> Self {
        Self {
            x: ComponentRange::new(),
            m: ComponentRange::new(),
            a: ComponentRange::new(),
            s: ComponentRange::new(),
            workflow: "in",
        }
    }

    fn get_component_range(&self, component: PartComponent) -> &ComponentRange {
        match component {
            PartComponent::X => &self.x,
            PartComponent::M => &self.m,
            PartComponent::A => &self.a,
            PartComponent::S => &self.s,
        }
    }

    fn with_component_range(
        &self,
        component: PartComponent,
        component_range: ComponentRange,
        next_workflow: &'a str,
    ) -> Self {
        let mut new = self.clone();
        match component {
            PartComponent::X => new.x = component_range,
            PartComponent::M => new.m = component_range,
            PartComponent::A => new.a = component_range,
            PartComponent::S => new.s = component_range,
        }
        new.workflow = next_workflow;
        new
    }
}

pub fn soln() -> (u32, u64) {
    let input = include_str!("../input/19.txt");
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|workflow| {
            let (name, rules) = workflow.split_once('{').unwrap();
            let (_, rules) = rules.as_bytes().split_last().unwrap();
            let rules = std::str::from_utf8(rules)
                .unwrap()
                .split(',')
                .map(|rule| {
                    if let Some((condition, dest)) = rule.split_once(':') {
                        let (component, gt, comp_target) =
                            if let Some((component, target)) = condition.split_once('<') {
                                (
                                    PartComponent::from_str(component),
                                    false,
                                    target.parse::<u32>().unwrap(),
                                )
                            } else {
                                let (component, target) = condition.split_once('>').unwrap();
                                (
                                    PartComponent::from_str(component),
                                    true,
                                    target.parse::<u32>().unwrap(),
                                )
                            };
                        Rule {
                            condition: Some(Condition {
                                component,
                                gt,
                                comp_target,
                            }),
                            dest,
                        }
                    } else {
                        Rule {
                            condition: None,
                            dest: rule,
                        }
                    }
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect::<HashMap<&str, Vec<Rule>>>();

    let parts = parts
        .lines()
        .map(|part| {
            let groups = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}")
                .unwrap()
                .captures(part)
                .unwrap();
            let (x, m, a, s) = (
                groups.get(1).unwrap().as_str().parse().unwrap(),
                groups.get(2).unwrap().as_str().parse().unwrap(),
                groups.get(3).unwrap().as_str().parse().unwrap(),
                groups.get(4).unwrap().as_str().parse().unwrap(),
            );
            Part { x, m, a, s }
        })
        .collect::<Vec<_>>();

    let p1 = parts
        .iter()
        .map(|part| {
            let mut current_workflow = "in";
            let accepted = loop {
                if current_workflow == "A" || current_workflow == "R" {
                    break current_workflow == "A";
                }
                let workflow = workflows.get(current_workflow).unwrap();
                for rule in workflow {
                    let applies = if let Some(condition) = &rule.condition {
                        let component_value = part.get_component(&condition.component);
                        if condition.gt {
                            component_value > condition.comp_target
                        } else {
                            component_value < condition.comp_target
                        }
                    } else {
                        true
                    };
                    if applies {
                        current_workflow = rule.dest;
                        break;
                    }
                }
            };
            if accepted {
                part.rating_sum()
            } else {
                0
            }
        })
        .sum();

    let mut part_ranges = vec![PartRange::new()];
    let mut accepted = Vec::new();
    while let Some(mut current) = part_ranges.pop() {
        if current.workflow == "A" {
            accepted.push(current);
            continue;
        } else if current.workflow == "R" {
            continue;
        }
        let workflow = workflows.get(current.workflow).unwrap();
        for rule in workflow {
            if let Some(condition) = &rule.condition {
                let component_range = current.get_component_range(condition.component);
                if condition.gt && component_range.end <= condition.comp_target
                    || (!condition.gt && component_range.start >= condition.comp_target)
                {
                    // Range falls entirely before a `>` condition, or after a `<` condition.
                    continue;
                }

                let before_range = (
                    component_range.start,
                    if condition.gt {
                        condition.comp_target
                    } else {
                        condition.comp_target - 1
                    },
                );
                let after_range = (
                    if condition.gt {
                        condition.comp_target + 1
                    } else {
                        condition.comp_target
                    },
                    component_range.end,
                );
                if before_range.0 < before_range.1 {
                    let next_part_range = current.with_component_range(
                        condition.component,
                        ComponentRange {
                            start: before_range.0,
                            end: before_range.1,
                        },
                        if condition.gt {
                            current.workflow
                        } else {
                            rule.dest
                        },
                    );
                    part_ranges.push(next_part_range);
                }
                if after_range.0 < after_range.1 {
                    let next_part_range = current.with_component_range(
                        condition.component,
                        ComponentRange {
                            start: after_range.0,
                            end: after_range.1,
                        },
                        if condition.gt {
                            rule.dest
                        } else {
                            current.workflow
                        },
                    );
                    part_ranges.push(next_part_range);
                }
                break;
            } else {
                // Whole range gets the new dest.
                current.workflow = rule.dest;
                part_ranges.push(current);
                break;
            };
        }
    }

    let p2 = accepted
        .iter()
        .map(|part_range| {
            (part_range.x.end - part_range.x.start + 1) as u64
                * (part_range.m.end - part_range.m.start + 1) as u64
                * (part_range.a.end - part_range.a.start + 1) as u64
                * (part_range.s.end - part_range.s.start + 1) as u64
        })
        .sum::<u64>();

    (p1, p2)
}
