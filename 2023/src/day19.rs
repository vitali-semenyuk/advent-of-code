use std::{collections::HashMap, fmt::Display, ops::Range};

#[derive(Clone, Copy, Debug)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::ExtremelyCoolLooking,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Invalid category value"),
        }
    }
}

#[derive(Debug)]
struct Part {
    extremely_cool_looking: u32,
    musical: u32,
    aerodynamic: u32,
    shiny: u32,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut part = Self {
            extremely_cool_looking: 0,
            musical: 0,
            aerodynamic: 0,
            shiny: 0,
        };

        for prop in value
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
        {
            let (name, value) = prop.split_once('=').unwrap();
            let value = value.parse().unwrap();

            match name {
                "x" => part.extremely_cool_looking = value,
                "m" => part.musical = value,
                "a" => part.aerodynamic = value,
                "s" => part.shiny = value,
                _ => panic!("Invalid part property"),
            }
        }

        part
    }
}

impl Part {
    fn get_value(&self, category: Category) -> u32 {
        match category {
            Category::ExtremelyCoolLooking => self.extremely_cool_looking,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }

    fn sum(&self) -> u32 {
        self.extremely_cool_looking + self.musical + self.aerodynamic + self.shiny
    }
}

#[derive(Clone, Debug)]
struct PartsRange {
    extremely_cool_looking: Range<u32>,
    musical: Range<u32>,
    aerodynamic: Range<u32>,
    shiny: Range<u32>,
}

impl Default for PartsRange {
    fn default() -> Self {
        Self {
            extremely_cool_looking: 1..4001,
            musical: 1..4001,
            aerodynamic: 1..4001,
            shiny: 1..4001,
        }
    }
}

impl PartsRange {
    fn combinations_count(&self) -> u64 {
        let extremely_cool_looking = self.extremely_cool_looking.clone().count() as u64;
        let musical = self.musical.clone().count() as u64;
        let aerodynamic = self.aerodynamic.clone().count() as u64;
        let shiny = self.shiny.clone().count() as u64;

        extremely_cool_looking * musical * aerodynamic * shiny
    }
}

#[derive(Debug)]
enum Comparison {
    Greater,
    Less,
}

impl From<char> for Comparison {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => panic!("Invalid comparison value"),
        }
    }
}

#[derive(Debug)]
struct Condition {
    comparison: Comparison,
    category: Category,
    value: u32,
}

impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        let category = chars.next().unwrap().into();
        let comparison = chars.next().unwrap().into();
        let value = chars.as_str().parse().unwrap();

        Self {
            comparison,
            category,
            value,
        }
    }
}

impl Condition {
    fn check(&self, part: &Part) -> bool {
        let value = part.get_value(self.category);

        match self.comparison {
            Comparison::Greater => value > self.value,
            Comparison::Less => value < self.value,
        }
    }

    // 0 - 100
    // > 40
    // 0..41, 41..101
    // < 40
    // 0..40, 40..101

    fn check_range(&self, range: &PartsRange) -> (PartsRange, PartsRange) {
        let mut less = range.clone();
        let mut greater = range.clone();

        let value = match self.comparison {
            Comparison::Greater => self.value + 1,
            Comparison::Less => self.value,
        };

        match self.category {
            Category::ExtremelyCoolLooking => {
                less.extremely_cool_looking = range.extremely_cool_looking.start..value;
                greater.extremely_cool_looking = value..range.extremely_cool_looking.end;
            }
            Category::Musical => {
                less.musical = range.musical.start..value;
                greater.musical = value..range.musical.end;
            }
            Category::Aerodynamic => {
                less.aerodynamic = range.aerodynamic.start..value;
                greater.aerodynamic = value..range.aerodynamic.end;
            }
            Category::Shiny => {
                less.shiny = range.shiny.start..value;
                greater.shiny = value..range.shiny.end;
            }
        }

        match self.comparison {
            Comparison::Greater => (greater, less),
            Comparison::Less => (less, greater),
        }
    }
}

#[derive(Clone, Debug)]
enum RuleResult {
    Workflow(String),
    Accepted,
    Rejected,
}

impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            workflow => Self::Workflow(workflow.to_string()),
        }
    }
}

#[derive(Debug)]
struct Rule {
    result: RuleResult,
    condition: Option<Condition>,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some((condition, result)) = value.split_once(':') {
            let condition = Some(Condition::from(condition));
            let result = RuleResult::from(result);

            Self { condition, result }
        } else {
            let condition = None;
            let result = RuleResult::from(value);

            Self { condition, result }
        }
    }
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<RuleResult> {
        if let Some(condition) = &self.condition {
            if condition.check(part) {
                Some(self.result.clone())
            } else {
                None
            }
        } else {
            Some(self.result.clone())
        }
    }

    fn apply_to_range(&self, range: &PartsRange) -> Vec<(PartsRange, Option<RuleResult>)> {
        if let Some(condition) = &self.condition {
            let (applicable, notapplicable) = condition.check_range(range);
            vec![
                (applicable, Some(self.result.clone())),
                (notapplicable, None),
            ]
        } else {
            vec![(range.clone(), Some(self.result.clone()))]
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rules) = value.strip_suffix('}').unwrap().split_once('{').unwrap();
        let name = name.to_string();
        let rules = rules.split(',').map(Rule::from).collect();

        Self { name, rules }
    }
}

impl Workflow {
    fn process(&self, part: &Part) -> RuleResult {
        self.rules
            .iter()
            .flat_map(|rule| rule.apply(part))
            .next()
            .unwrap()
    }

    fn process_range(&self, range: &PartsRange) -> Vec<(PartsRange, RuleResult)> {
        self.rules
            .iter()
            .fold(vec![(range.clone(), None)], |acc, rule| {
                let (unprocessed, mut processed): (Vec<_>, Vec<_>) =
                    acc.into_iter().partition(|(_, result)| result.is_none());

                let new = unprocessed
                    .into_iter()
                    .flat_map(|(range, _)| rule.apply_to_range(&range));
                processed.extend(new);
                processed
            })
            .into_iter()
            .map(|(range, result)| (range, result.unwrap()))
            .collect()
    }
}

#[derive(Debug)]
struct System(HashMap<String, Workflow>);

impl From<&str> for System {
    fn from(value: &str) -> Self {
        let workflows = value
            .lines()
            .map(Workflow::from)
            .map(|w| (w.name.clone(), w))
            .collect();

        Self(workflows)
    }
}

impl System {
    fn process(&self, part: &Part) -> bool {
        self.process_workflow(part, "in")
    }

    fn process_workflow(&self, part: &Part, workflow: &str) -> bool {
        let current_workflow = self.0.get(workflow).unwrap();

        match current_workflow.process(part) {
            RuleResult::Workflow(next_workflow) => self.process_workflow(part, &next_workflow),
            RuleResult::Accepted => true,
            RuleResult::Rejected => false,
        }
    }

    fn process_range(&self, range: &PartsRange) -> u64 {
        self.process_range_workflow(range, "in")
            .into_iter()
            .flatten()
            .map(|r| r.combinations_count())
            .sum()
    }

    fn process_range_workflow(
        &self,
        range: &PartsRange,
        workflow: &str,
    ) -> Vec<Option<PartsRange>> {
        let current_workflow = self.0.get(workflow).unwrap();

        current_workflow
            .process_range(range)
            .into_iter()
            .flat_map(|(rng, res)| match res {
                RuleResult::Workflow(next_workflow) => {
                    self.process_range_workflow(&rng, &next_workflow)
                }
                RuleResult::Accepted => vec![Some(rng)],
                RuleResult::Rejected => vec![None],
            })
            .collect()
    }
}

pub fn solve(input: &str) -> (Box<dyn Display>, Box<dyn Display>) {
    (
        Box::new(solve_first_part(input)),
        Box::new(solve_second_part(input)),
    )
}

fn solve_first_part(input: &str) -> u32 {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let system = System::from(workflows);
    parts
        .lines()
        .map(Part::from)
        .filter(|part| system.process(part))
        .map(|part| part.sum())
        .sum()
}

fn solve_second_part(input: &str) -> u64 {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let system = System::from(workflows);

    system.process_range(&PartsRange::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_first_part() {
        let answer = 19114;

        assert_eq!(answer, solve_first_part(INPUT))
    }

    #[test]
    fn test_second_part() {
        let answer = 167409079868000;

        assert_eq!(answer, solve_second_part(INPUT))
    }

    check_answers!(495298, 132186256794011);
}
